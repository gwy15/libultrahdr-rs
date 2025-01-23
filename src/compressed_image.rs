use libultrahdr_sys as sys;

pub struct CompressedImage<'a> {
    pub inner: sys::uhdr_compressed_image,
    _marker: &'a (),
}
impl<'a> CompressedImage<'a> {
    pub fn from_bytes(bytes: &'_ mut [u8]) -> Self {
        Self {
            inner: sys::uhdr_compressed_image {
                data: bytes.as_mut_ptr().cast(),
                data_sz: bytes.len(),
                capacity: bytes.len(),
                cg: sys::uhdr_color_gamut::UHDR_CG_UNSPECIFIED,
                ct: sys::uhdr_color_transfer::UHDR_CT_UNSPECIFIED,
                range: sys::uhdr_color_range::UHDR_CR_FULL_RANGE,
            },
            _marker: &(),
        }
    }
    pub unsafe fn from_mut_ptr(ptr: *mut sys::uhdr_compressed_image) -> CompressedImage<'a> {
        Self {
            inner: unsafe { *ptr },
            _marker: &(),
        }
    }
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            let ptr = self.inner.data.cast::<u8>();
            let len = self.inner.data_sz;
            std::slice::from_raw_parts(ptr, len)
        }
    }
    pub fn color_gamut(&self) -> sys::uhdr_color_gamut {
        self.inner.cg
    }
    pub fn color_gamut_mut(&mut self) -> &mut sys::uhdr_color_gamut {
        &mut self.inner.cg
    }
    pub fn color_transfer(&self) -> sys::uhdr_color_transfer {
        self.inner.ct
    }
    pub fn color_transfer_mut(&mut self) -> &mut sys::uhdr_color_transfer {
        &mut self.inner.ct
    }
    pub fn color_range(&self) -> sys::uhdr_color_range {
        self.inner.range
    }
    pub fn color_range_mut(&mut self) -> &mut sys::uhdr_color_range {
        &mut self.inner.range
    }
    pub fn as_ptr(&self) -> *const sys::uhdr_compressed_image {
        &self.inner
    }
    pub fn as_mut_ptr(&mut self) -> *mut sys::uhdr_compressed_image {
        &mut self.inner
    }
}
