use x11::xlib::{Window, GC};

///
/// ```text
/// typedef struct	s_win_list
/// {
/// 	Window				window;
/// 	GC					gc;
/// 	struct s_win_list	*next;
/// 	int					(*mouse_hook)();
/// 	int					(*key_hook)();
/// 	int					(*expose_hook)();
/// 	void				*mouse_param;
/// 	void				*key_param;
/// 	void				*expose_param;
/// 	t_event_list		hooks[MLX_MAX_EVENT];
/// }				t_win_list;
///
/// ```
///

#[derive(Debug)]
pub struct MlxWindow {
    window: Window,
    gc: GC,
}

impl MlxWindow {
    pub fn new() -> Self {
        unimplemented!()
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
