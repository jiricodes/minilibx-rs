//! Image module
//! 

/// ```text
/// typedef struct	s_img
/// {
/// 	XImage			*image;
/// 	Pixmap			pix;
/// 	GC				gc;
/// 	int				size_line;
/// 	int				bpp;
/// 	int				width;
/// 	int				height;
/// 	int				type;
/// 	int				format;
/// 	char			*data;
/// 	XShmSegmentInfo	shm;
/// }				t_img;
/// ```