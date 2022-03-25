//! Mlx module, main struct and window handling here
//!
//!

use std::ffi::CString;
use std::mem::MaybeUninit;
use std::ptr;

use x11::xlib::{
    CWBackPixel, CWBorderPixel, CWColormap, CWEventMask, Colormap, CopyFromParent, Depth, Display,
    InputOutput, PMaxSize, PMinSize, PPosition, PSize, Screen, Visual, Window, XCreateWindow,
    XGCValues, XSetWMNormalHints, XSetWindowAttributes, XSizeHints, XStoreName,
};
use x11::{xlib, xshm};

use crate::MlxWindow;

/// Main mlx struct
///
/// TODO:
/// - depth as `xlib::Depth`
///
/// ```text
/// typedef struct	s_xvar
/// {
/// 	Display		*display;
/// 	Window		root;
/// 	int			screen;
/// 	int			depth;
/// 	Visual		*visual;
/// 	Colormap	cmap;
/// 	int			private_cmap;
/// 	t_win_list	*win_list;
/// 	int			(*loop_hook)();
/// 	void		*loop_param;
/// 	int			use_xshm;
/// 	int			pshm_format;
/// 	int			do_flush;
/// 	int			decrgb[6];
/// 	Atom		wm_delete_window;
/// 	Atom		wm_protocols;
/// 	int 		end_loop;
/// }				t_xvar;
/// ```
#[derive(Debug)]
pub struct Mlx {
    display: *mut Display,
    screen: i32,
    root: Window,
    cmap: Colormap,
    depth: i32,
    visual: *mut Visual,
    windows: Vec<MlxWindow>,
    use_xshm: bool,
    pshm_format: i32,
    do_flush: bool,
    decrgb: [i32; 6],
    end_loop: bool,
}

impl Mlx {
    /// Initialises mlx - similar to `void *mlx_init();`
    pub fn new() -> Self {
        let mut new = Self::default();
        let ret = new.int_get_visual();
        assert!(ret == 0, "int_get_visual non-zero return");
        new.int_deal_shm();
        new.int_rgb_conversion();
        new
    }

    ///`int		mlx_int_get_visual(t_xvar *xvar)`
    /// Change to return Result instead
    fn int_get_visual(&mut self) -> i32 {
        // Handle if visual class is not TrueColor
        // Error for some reason `Visual` doenst seem to have `class`
        // https://docs.rs/x11/2.19.1/x11/xlib/struct.Visual.html
        // if *visual.class != xlib::TrueColor {

        // }
        0
    }

    /// pshm_format of -1 :	Not XYBitmap|XYPixmap|ZPixmap
    /// alpha libX need a check of the DISPLAY env var, or shm is allowed
    /// in remote Xserver connections.
    ///
    /// `int		mlx_int_deal_shm(t_xvar *xvar)`
    fn int_deal_shm(&mut self) {
        use gethostname::gethostname;
        use std::env;

        let mut bidon: i32 = 0;
        let mut use_pshm: i32 = 0;
        self.use_xshm =
            unsafe { xshm::XShmQueryVersion(self.display, &mut bidon, &mut bidon, &mut use_pshm) }
                != 0;
        if self.use_xshm && use_pshm != 0 {
            self.pshm_format = unsafe { xshm::XShmPixmapFormat(self.display) };
        } else {
            self.pshm_format = -1;
        }
        let hostname = gethostname();
        let host_str = hostname
            .to_str()
            .expect("could not transform hostname to str");
        if let Ok(dpy) = env::var("DISPLAY") {
            if dpy.len() > 0
                && dpy.chars().nth(0).unwrap() != ':'
                && !dpy.starts_with(host_str)
                && !dpy.starts_with("localhost")
            {
                self.pshm_format = -1;
                self.use_xshm = false;
            }
        }
        return;
    }

    /// TrueColor Visual is needed to have *_mask correctly set
    /// `int		mlx_int_rgb_conversion(t_xvar *xvar)`
    fn int_rgb_conversion(&mut self) {
        // unimplemented!()
        return;
    }

    pub fn new_window(&mut self, size_x: u32, size_y: u32, title: &str) {
        let xswa = Box::new(XSetWindowAttributes {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: 0,
            border_pixel: u64::MAX,
            bit_gravity: 0,
            win_gravity: 0,
            backing_store: 0,
            backing_planes: 0,
            backing_pixel: 0,
            save_under: 0,
            event_mask: 0xFFFFFF, // all events
            do_not_propagate_mask: 0,
            override_redirect: 0,
            colormap: self.cmap,
            cursor: 0,
        });

        let new_window = unsafe {
            XCreateWindow(
                self.display,
                self.root,
                0,
                0,
                size_x,
                size_y,
                0,
                CopyFromParent,
                InputOutput as u32,
                self.visual,
                CWEventMask | CWBackPixel | CWBorderPixel | CWColormap,
                Box::into_raw(xswa),
            )
        };
        // Anti-resize
        // TODO: Should this be to toggle?
        self.anti_resize_win(new_window, size_x, size_y);
        // XStoreName
        let title = CString::new(title).unwrap();
        unsafe { XStoreName(self.display, new_window, title.as_ptr()) };
        // TODO:
        // XSetWMProtocols
        // Create GC
        // add to vec
        // hooks
        // xMApRaised
        // wait first expopse
        // return something significant xD
    }

    /// int	mlx_int_anti_resize_win(t_xvar *xvar,Window win,int w,int h)
    fn anti_resize_win(&mut self, window: Window, size_x: u32, size_y: u32) {
        let hints: XSizeHints = unsafe { MaybeUninit::<XSizeHints>::zeroed().assume_init() };
        let mut hints = Box::new(hints);
        hints.width = size_x as i32;
        hints.height = size_y as i32;
        hints.min_width = size_x as i32;
        hints.min_height = size_y as i32;
        hints.max_width = size_x as i32;
        hints.max_height = size_y as i32;
        hints.flags = PPosition | PSize | PMinSize | PMaxSize;
        unsafe { XSetWMNormalHints(self.display, window, Box::into_raw(hints)) };
    }
}

impl Default for Mlx {
    fn default() -> Self {
        // Open Display connection
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        assert!(!display.is_null(), "XOpenDisplay failed");

        // Create a window
        let screen = unsafe { xlib::XDefaultScreen(display) };
        let root = unsafe { xlib::XRootWindow(display, screen) };
        // Setup colors
        let cmap = unsafe { xlib::XDefaultColormap(display, screen) };

        let depth = unsafe { xlib::XDefaultDepth(display, screen) };

        let visual = unsafe { xlib::XDefaultVisual(display, screen) };

        Self {
            display,
            screen,
            root,
            cmap,
            depth,
            visual,
            windows: Vec::new(),
            use_xshm: false,
            pshm_format: -1,
            do_flush: true,
            decrgb: [0; 6],
            end_loop: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut mlx = Mlx::new();
        dbg!(&mlx);
        mlx.anti_resize_win(0, 10, 10);
    }
}
