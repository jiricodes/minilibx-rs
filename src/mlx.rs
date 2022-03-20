//! Mlx module, main struct and window handling here
//!
//!

use std::ptr;

use x11::xlib;
use x11::xlib::{Colormap, Depth, Display, Screen, Window, Visual};

/// Main mlx struct
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
    depth: Depth,
    visual: *mut Visual,
    private_cmap: bool,
    windows: Vec<MlxWindow>,
    use_xshm: i32,
    pshm_format: i32,

}

impl Mlx {
    /// Initialises mlx - similar to `void *mlx_init();`
    pub fn new() -> Self {
        // Open Display connection
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        assert!(!display.is_null(), "XOpenDisplay failed");

        // Create a window
        let screen = unsafe { xlib::XDefaultScreen(display) };
        let root = unsafe { xlib::XRootWindow(display, screen) };
        // Setup colors
        let cmap = unsafe { xlib::XDefaultColormap(display, screen) };

        let depth_i = unsafe { xlib::XDefaultDepth(display, screen) };
        
        // `int		mlx_int_get_visual(t_xvar *xvar)`
        let mut private_cmap = false;
        let visual = unsafe {xlib::XDefaultVisual(display, screen) };
        // Handle if visual class is not TrueColor
        // Error for some reason `Visual` doenst seem to have `class`
        // if *visual.class != xlib::TrueColor {

        // }
        private_cmap = true;

        let depth = Depth { 

        }

        Self {
            display,
            screen,
            root,
            cmap,
            depth,
            visual
        }
    }

    pub fn new_window() {
        unimplemented!()
    }
}

pub struct MlxWindow {}

impl MlxWindow {
    pub fn new() -> Self {
        Self {}
    }

    pub fn clear() {
        unimplemented!()
    }

    /// Origin for x & y is top left corner of the window
    /// y down is positive
    /// color is 0x00RRGGBB
    ///
    /// `int	mlx_pixel_put(void *mlx_ptr, void *win_ptr, int x, int y, int color);`
    pub fn pixel_put(&mut self, x: i32, y: i32, color: i32) {
        unimplemented!()
    }
}
