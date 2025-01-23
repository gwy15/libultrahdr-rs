use libultrahdr_sys as sys;

use crate::{CompressedImage, GainmapMetadata, RawImage, Result, UhdrError};

pub struct Encoder {
    pub(crate) ptr: *mut sys::uhdr_codec_private,
}
impl Encoder {
    pub fn new() -> Self {
        let ptr = unsafe { sys::uhdr_create_encoder() };
        Self { ptr }
    }

    /// Add raw image descriptor to encoder context.
    pub fn set_raw_image(
        &mut self,
        img: RawImage<'_>,
        intent: sys::uhdr_img_label_t,
    ) -> Result<()> {
        let mut c_img = img.into();
        let err = unsafe { sys::uhdr_enc_set_raw_image(self.ptr, &mut c_img, intent) };
        UhdrError::check(err)?;
        Ok(())
    }
    pub fn set_raw_sdr_image(&mut self, img: RawImage<'_>) -> Result<()> {
        self.set_raw_image(img, sys::uhdr_img_label_t::UHDR_SDR_IMG)
    }
    pub fn set_raw_hdr_image(&mut self, img: RawImage<'_>) -> Result<()> {
        self.set_raw_image(img, sys::uhdr_img_label_t::UHDR_HDR_IMG)
    }

    pub fn set_compressed_image(
        &mut self,
        compressed_image: &CompressedImage<'_>,
        intent: sys::uhdr_img_label_t,
    ) -> Result<()> {
        let err =
            unsafe { sys::uhdr_enc_set_compressed_image(self.ptr, compressed_image.ptr, intent) };
        UhdrError::check(err)?;
        Ok(())
    }
    pub fn set_compressed_base_image(
        &mut self,
        compressed_image: &CompressedImage<'_>,
    ) -> Result<()> {
        self.set_compressed_image(compressed_image, sys::uhdr_img_label_t::UHDR_BASE_IMG)
    }
    pub fn set_compressed_sdr_image(
        &mut self,
        compressed_image: &CompressedImage<'_>,
    ) -> Result<()> {
        self.set_compressed_image(compressed_image, sys::uhdr_img_label_t::UHDR_SDR_IMG)
    }
    pub fn set_compressed_hdr_image(
        &mut self,
        compressed_image: &CompressedImage<'_>,
    ) -> Result<()> {
        self.set_compressed_image(compressed_image, sys::uhdr_img_label_t::UHDR_HDR_IMG)
    }

    pub fn set_gainmap_image(
        &mut self,
        img: CompressedImage<'_>,
        metadata: GainmapMetadata,
    ) -> Result<()> {
        let mut metadata = metadata.into();
        let err = unsafe { sys::uhdr_enc_set_gainmap_image(self.ptr, img.ptr, &mut metadata) };
        UhdrError::check(err)?;
        Ok(())
    }
    // pub fn set_exif_data()
    // pub fn set_using_multi_channel_gainmap
    pub fn set_gainmap_scale_factor(
        &mut self,
        gainmap_scale_factor: std::ffi::c_int,
    ) -> Result<()> {
        let err = unsafe { sys::uhdr_enc_set_gainmap_scale_factor(self.ptr, gainmap_scale_factor) };
        UhdrError::check(err)?;
        Ok(())
    }
    pub fn set_gainmap_gamma(&mut self, gamma: f32) -> Result<()> {
        let err = unsafe { sys::uhdr_enc_set_gainmap_gamma(self.ptr, gamma) };
        UhdrError::check(err)?;
        Ok(())
    }
    pub fn set_min_max_content_boost(&mut self, min_boost: f32, max_boost: f32) -> Result<()> {
        let err =
            unsafe { sys::uhdr_enc_set_min_max_content_boost(self.ptr, min_boost, max_boost) };
        UhdrError::check(err)?;
        Ok(())
    }
    pub fn set_target_display_peak_brightness(&mut self, nits: f32) -> Result<()> {
        let err = unsafe { sys::uhdr_enc_set_target_display_peak_brightness(self.ptr, nits) };
        UhdrError::check(err)?;
        Ok(())
    }
    // pub fn set_preset()
    // pub fn set_output_format()

    /// Set quality factor for compressing base image.
    /// Default configured quality factor is 95.
    ///
    /// # Arguments
    /// - quality: quality factor. Any integer in range [0 - 100].
    pub fn set_base_image_quality(&mut self, quality: i32) -> Result<()> {
        let err = unsafe {
            sys::uhdr_enc_set_quality(self.ptr, quality, sys::uhdr_img_label_t::UHDR_BASE_IMG)
        };
        UhdrError::check(err)?;
        Ok(())
    }
    /// Set quality factor for compressing gainmap image.
    /// Default configured quality factor is 95.
    ///
    /// # Arguments
    /// - quality: quality factor. Any integer in range [0 - 100].
    pub fn set_gainmap_image_quality(&mut self, quality: i32) -> Result<()> {
        let err = unsafe {
            sys::uhdr_enc_set_quality(self.ptr, quality, sys::uhdr_img_label_t::UHDR_GAIN_MAP_IMG)
        };
        UhdrError::check(err)?;
        Ok(())
    }

    pub fn encode(&mut self) -> Result<()> {
        let err = unsafe { sys::uhdr_encode(self.ptr) };
        UhdrError::check(err)?;
        Ok(())
    }
    pub fn get_encoded_stream(&mut self) -> Option<CompressedImage<'_>> {
        let err = unsafe { sys::uhdr_get_encoded_stream(self.ptr) };
        if err.is_null() {
            return None;
        }
        Some(unsafe { CompressedImage::from_ptr(err) })
    }
}
impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { sys::uhdr_release_encoder(self.ptr) };
    }
}
