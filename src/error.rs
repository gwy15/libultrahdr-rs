use std::ffi::CString;

use libultrahdr_sys::uhdr_error_info;

pub struct UhdrError {
    pub error_code: libultrahdr_sys::uhdr_codec_err_t,
    pub detail: Option<CString>,
}
impl UhdrError {
    pub fn check(raw: uhdr_error_info) -> Result<(), Self> {
        if raw.error_code == libultrahdr_sys::uhdr_codec_err_t::UHDR_CODEC_OK {
            return Ok(());
        }
        let detail = if raw.has_detail == 0 {
            None
        } else {
            Some(unsafe {
                let s = std::ffi::CStr::from_ptr(raw.detail.as_ptr());
                s.to_owned()
            })
        };
        let this = Self {
            error_code: raw.error_code,
            detail,
        };
        Err(this)
    }
}

impl std::fmt::Debug for UhdrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UhdrError")
            .field("error_code", &self.error_code)
            .field("detail", &self.detail)
            .finish()
    }
}

impl std::fmt::Display for UhdrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UhdrError({:?})", self.error_code)?;
        if let Some(detail) = &self.detail {
            write!(f, " {}", detail.to_string_lossy())?;
        }
        Ok(())
    }
}
impl std::error::Error for UhdrError {}

pub type Result<T, E = UhdrError> = std::result::Result<T, E>;
