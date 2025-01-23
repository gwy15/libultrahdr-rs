use libultrahdr_sys as sys;

pub struct RawImage<'a> {
    /// Image Format
    pub fmt: sys::uhdr_img_fmt_t,
    /// Color Gamut
    pub color_gamut: sys::uhdr_color_gamut_t,
    /// Color Transfer
    pub color_transfer: sys::uhdr_color_transfer_t,
    /// Color Range
    pub range: sys::uhdr_color_range_t,
    pub width: u32,
    pub height: u32,
    pub planes: [&'a mut [u8]; 3],
    pub stride: [std::ffi::c_uint; 3],
}

pub unsafe fn parse_planes<'a>(raw_image: sys::uhdr_raw_image_t) -> [&'a mut [u8]; 3] {
    let mut planes = [&mut [] as &mut _, &mut [] as _, &mut [] as _];
    let (w, h) = (raw_image.w as usize, raw_image.h as usize);
    const UHDR_PLANE_Y: usize = sys::UHDR_PLANE_Y as usize;
    const UHDR_PLANE_UV: usize = sys::UHDR_PLANE_UV as usize;
    const UHDR_PLANE_U: usize = sys::UHDR_PLANE_U as usize;
    const UHDR_PLANE_V: usize = sys::UHDR_PLANE_V as usize;
    const UHDR_PLANE_PACKED: usize = sys::UHDR_PLANE_PACKED as usize;

    macro_rules! read {
        ($plane:expr, $len:expr) => {
            planes[$plane] = std::slice::from_raw_parts_mut(raw_image.planes[$plane].cast(), $len);
        };
    }

    unsafe {
        match raw_image.fmt {
            sys::uhdr_img_fmt::UHDR_IMG_FMT_UNSPECIFIED => {}
            sys::uhdr_img_fmt::UHDR_IMG_FMT_24bppYCbCrP010 => {
                let bpp = 2;
                read!(UHDR_PLANE_Y, bpp * w * h);
                assert_eq!(raw_image.stride[UHDR_PLANE_Y], raw_image.w / 2);
                read!(UHDR_PLANE_UV, bpp * w / 2 * h / 2 * 2);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_12bppYCbCr420 => {
                read!(UHDR_PLANE_Y, w * h);
                read!(UHDR_PLANE_U, w / 2 * h / 2);
                read!(UHDR_PLANE_V, w / 2 * h / 2);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_8bppYCbCr400 => {
                read!(UHDR_PLANE_Y, w * h);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_32bppRGBA8888 => {
                read!(UHDR_PLANE_PACKED, w * h * 4);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_64bppRGBAHalfFloat => {
                read!(UHDR_PLANE_PACKED, w * h * 8);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_32bppRGBA1010102 => {
                read!(UHDR_PLANE_PACKED, w * h * 4);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_24bppYCbCr444 => {
                read!(UHDR_PLANE_Y, w * h);
                read!(UHDR_PLANE_U, w * h);
                read!(UHDR_PLANE_V, w * h);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_16bppYCbCr422 => {
                read!(UHDR_PLANE_Y, w * h);
                // 4:2:2,
                read!(UHDR_PLANE_U, w / 2 * h);
                read!(UHDR_PLANE_V, w / 2 * h);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_16bppYCbCr440 => {
                read!(UHDR_PLANE_Y, w * h);
                // 4:4:0, first line 4, second line 0. width is full whereas height is half
                read!(UHDR_PLANE_U, w * h / 2);
                read!(UHDR_PLANE_V, w * h / 2);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_12bppYCbCr411 => {
                // 4:1:1, first line 1, second line 1. width is 1/4 whereas height is full
                read!(UHDR_PLANE_Y, w * h);
                read!(UHDR_PLANE_U, w / 4 * h);
                read!(UHDR_PLANE_V, w / 4 * h);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_10bppYCbCr410 => {
                // 4:1:0
                read!(UHDR_PLANE_Y, w * h);
                read!(UHDR_PLANE_U, w / 4 * h / 2);
                read!(UHDR_PLANE_V, w / 4 * h / 2);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_24bppRGB888 => {
                read!(UHDR_PLANE_PACKED, w * h * 3);
            }
            sys::uhdr_img_fmt::UHDR_IMG_FMT_30bppYCbCr444 => {
                let bpp = 2;
                read!(UHDR_PLANE_Y, bpp * w * h);
                read!(UHDR_PLANE_U, bpp * w * h);
                read!(UHDR_PLANE_V, bpp * w * h);
            }
            _ => panic!("unsupported format: {}", raw_image.fmt as i32),
        }
    }
    planes
}

impl<'a> From<sys::uhdr_raw_image_t> for RawImage<'a> {
    fn from(c_image: sys::uhdr_raw_image_t) -> Self {
        let planes = unsafe { parse_planes(c_image) };
        Self {
            fmt: c_image.fmt,
            color_gamut: c_image.cg,
            color_transfer: c_image.ct,
            range: c_image.range,
            width: c_image.w,
            height: c_image.h,
            planes,
            stride: c_image.stride,
        }
    }
}
impl<'a> From<RawImage<'a>> for sys::uhdr_raw_image_t {
    fn from(raw_image: RawImage<'a>) -> sys::uhdr_raw_image_t {
        let mut c_image = sys::uhdr_raw_image_t {
            fmt: raw_image.fmt,
            cg: raw_image.color_gamut,
            ct: raw_image.color_transfer,
            range: raw_image.range,
            w: raw_image.width,
            h: raw_image.height,
            planes: [std::ptr::null_mut(); 3],
            stride: raw_image.stride,
        };
        for i in 0..3 {
            c_image.planes[i] = raw_image.planes[i].as_mut_ptr().cast();
        }
        c_image
    }
}
