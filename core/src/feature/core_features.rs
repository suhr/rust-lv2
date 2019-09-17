//! Contains the LV2 features defined by the LV2 Core specification.
//!
//! This module is for internal organization only and is not meant to be exposed.

use crate::feature::Feature;
use crate::UriBound;

/// Marker feature to signal that the plugin can run in a hard real-time environment.
pub struct HardRTCapable;

unsafe impl UriBound for HardRTCapable {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__hardRTCapable;
}

unsafe impl<'a> Feature<'a> for HardRTCapable {}

/// Marker feature to signal the host to avoid in-place operation.
///
/// This feature has to be required by any plugin that may break if ANY input port is connected to the same memory location as ANY output port.
pub struct InPlaceBroken;

unsafe impl UriBound for InPlaceBroken {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__inPlaceBroken;
}

unsafe impl<'a> Feature<'a> for InPlaceBroken {}

/// Marker feature to signal the host to only run the plugin in a live environment.
pub struct IsLive;

unsafe impl UriBound for IsLive {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__isLive;
}

unsafe impl<'a> Feature<'a> for IsLive {}