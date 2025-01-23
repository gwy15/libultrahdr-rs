use libultrahdr_sys as sys;

#[derive(Clone)]
pub struct RawImage<T> {
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
    pub planes: [T; 3],
    pub stride: [std::ffi::c_uint; 3],
}

pub type OwnedRawImage = RawImage<Vec<u8>>;
pub type BorrowedRawImage<'a> = RawImage<&'a [u8]>;
pub type MutRawImage<'a> = RawImage<&'a mut [u8]>;

impl MutRawImage<'_> {
    pub fn from_c(c_image: sys::uhdr_raw_image_t) -> Self {
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
    pub fn into_c(self) -> sys::uhdr_raw_image_t {
        let mut c_image = sys::uhdr_raw_image_t {
            fmt: self.fmt,
            cg: self.color_gamut,
            ct: self.color_transfer,
            range: self.range,
            w: self.width,
            h: self.height,
            planes: [std::ptr::null_mut(); 3],
            stride: self.stride,
        };
        for i in 0..3 {
            c_image.planes[i] = self.planes[i].as_mut_ptr().cast();
        }
        c_image
    }
}
impl OwnedRawImage {
    pub fn new() -> Self {
        Self {
            fmt: sys::uhdr_img_fmt_t::UHDR_IMG_FMT_UNSPECIFIED,
            color_gamut: sys::uhdr_color_gamut::UHDR_CG_UNSPECIFIED,
            color_transfer: sys::uhdr_color_transfer_t::UHDR_CT_UNSPECIFIED,
            range: sys::uhdr_color_range_t::UHDR_CR_UNSPECIFIED,
            width: 0,
            height: 0,
            planes: [vec![], vec![], vec![]],
            stride: [0, 0, 0],
        }
    }
    pub fn as_ref(&self) -> BorrowedRawImage<'_> {
        BorrowedRawImage {
            fmt: self.fmt,
            color_gamut: self.color_gamut,
            color_transfer: self.color_transfer,
            range: self.range,
            width: self.width,
            height: self.height,
            planes: [&self.planes[0], &self.planes[1], &self.planes[2]],
            stride: self.stride,
        }
    }
    pub fn as_mut(&mut self) -> MutRawImage<'_> {
        MutRawImage {
            fmt: self.fmt,
            color_gamut: self.color_gamut,
            color_transfer: self.color_transfer,
            range: self.range,
            width: self.width,
            height: self.height,
            planes: unsafe {
                [
                    std::slice::from_raw_parts_mut(
                        self.planes[0].as_mut_ptr(),
                        self.planes[0].len(),
                    ),
                    std::slice::from_raw_parts_mut(
                        self.planes[1].as_mut_ptr(),
                        self.planes[1].len(),
                    ),
                    std::slice::from_raw_parts_mut(
                        self.planes[2].as_mut_ptr(),
                        self.planes[2].len(),
                    ),
                ]
            },
            stride: self.stride,
        }
    }
}

impl BorrowedRawImage<'_> {
    pub fn to_owned(&self) -> OwnedRawImage {
        OwnedRawImage {
            fmt: self.fmt,
            color_gamut: self.color_gamut,
            color_transfer: self.color_transfer,
            range: self.range,
            width: self.width,
            height: self.height,
            planes: [
                self.planes[0].to_vec(),
                self.planes[1].to_vec(),
                self.planes[2].to_vec(),
            ],
            stride: self.stride,
        }
    }
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
