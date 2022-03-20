//! Mlx module, main struct and window handling here
//!
//!

use std::ptr;

use x11::xlib;
use x11::xlib::{Colormap, Depth, Display, Screen, Visual, Window};

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
        unimplemented!()
    }

    /// TrueColor Visual is needed to have *_mask correctly set
    /// `int		mlx_int_rgb_conversion(t_xvar *xvar)`
    fn int_rgb_conversion(&mut self) {
        unimplemented!()
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
