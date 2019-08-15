//! LV2 specification centered around the Map feature.
//! 
//! The URID specification provides a host feature that can be used by plugins to map URIs to integers, so-called URIDs. These URIDs are used by many other specifications to identify other URI bounds and combine the flexibility of URIs with the comparison speed of integers.
//! 
//! Since this crate depends on `-sys` crates that use `bindgen` to create the C API bindings,
//! you need to have clang installed on your machine.
extern crate lv2_core as core;
pub extern crate lv2_urid_sys as sys;

pub mod feature;
mod urid;

pub use urid::*;
