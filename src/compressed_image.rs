use libultrahdr_sys as sys;

pub struct CompressedImage<'a> {
    pub ptr: *mut sys::uhdr_compressed_image,
    _marker: &'a (),
}
impl CompressedImage<'_> {
    pub unsafe fn from_ptr(ptr: *mut sys::uhdr_compressed_image) -> Self {
        CompressedImage { ptr, _marker: &() }
    }
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = (*self.ptr).data.cast::<u8>();
            let len = (*self.ptr).data_sz;
            std::slice::from_raw_parts(ptr, len)
        }
    }
    pub fn color_gamut(&self) -> sys::uhdr_color_gamut {
        unsafe { (*self.ptr).cg }
    }
    pub fn color_transfer(&self) -> sys::uhdr_color_transfer {
        unsafe { (*self.ptr).ct }
    }
    pub fn color_range(&self) -> sys::uhdr_color_range {
        unsafe { (*self.ptr).range }
    }
}
