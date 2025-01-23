use libultrahdr_sys as sys;

/// Gain map metadata
pub struct GainmapMetadata {
    /// Value to control how much brighter an image can get, when shown on an HDR display,
    /// relative to the SDR rendition. This is constant for a given image. Value MUST be in linear scale.
    pub max_content_boost: [f32; 3],
    /// Value to control how much darker an image can get, when shown on an HDR display,
    /// relative to the SDR rendition. This is constant for a given image. Value MUST be in linear scale.
    pub min_content_boost: [f32; 3],
    /// Encoding Gamma of the gainmap image.
    pub gamma: [f32; 3],
    /// The offset to apply to the SDR pixel values during gainmap generation and application.
    pub offset_sdr: [f32; 3],
    /// The offset to apply to the HDR pixel values during gainmap generation and application.
    pub offset_hdr: [f32; 3],
    /// Minimum display boost value for which the map is applied completely. Value MUST be in linear scale.
    pub hdr_capacity_min: f32,
    /// Maximum display boost value for which the map is applied completely.\nValue MUST be in linear scale.
    pub hdr_capacity_max: f32,
    /// Is gainmap application space same as base image color space
    pub use_base_cg: std::ffi::c_int,
}
impl From<sys::uhdr_gainmap_metadata> for GainmapMetadata {
    fn from(value: sys::uhdr_gainmap_metadata) -> Self {
        Self {
            max_content_boost: value.max_content_boost,
            min_content_boost: value.min_content_boost,
            gamma: value.gamma,
            offset_sdr: value.offset_sdr,
            offset_hdr: value.offset_hdr,
            hdr_capacity_min: value.hdr_capacity_min,
            hdr_capacity_max: value.hdr_capacity_max,
            use_base_cg: value.use_base_cg,
        }
    }
}
impl From<GainmapMetadata> for sys::uhdr_gainmap_metadata {
    fn from(value: GainmapMetadata) -> Self {
        Self {
            max_content_boost: value.max_content_boost,
            min_content_boost: value.min_content_boost,
            gamma: value.gamma,
            offset_sdr: value.offset_sdr,
            offset_hdr: value.offset_hdr,
            hdr_capacity_min: value.hdr_capacity_min,
            hdr_capacity_max: value.hdr_capacity_max,
            use_base_cg: value.use_base_cg,
        }
    }
}
