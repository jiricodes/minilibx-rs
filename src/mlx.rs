//! Mlx module, main struct and window handling here
//!
//!

use std::ptr;

use x11::xlib::{Colormap, Depth, Display, Screen, Visual, Window};
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

    pub fn new_window() {
        unimplemented!()
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
        let mlx = Mlx::new();
        dbg!(mlx);
    }
}
