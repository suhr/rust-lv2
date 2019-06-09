use crate::port::PortType;
use crate::uri::UriBound;
use std::ptr::NonNull;

pub struct Control {
    pointer: NonNull<f32>,
}

impl Control {
    pub fn value(&self) -> f32 {
        unsafe { *self.pointer.as_ptr() }
    }
}

unsafe impl UriBound for Control {
    const URI: &'static [u8] = ::lv2_core_sys::LV2_CORE__ControlPort;
}

impl PortType for Control {
    const NAME: &'static str = "Control";

    type InputPortType = Self;
    type OutputPortType = Self;

    #[inline]
    unsafe fn input_from_raw(pointer: NonNull<()>, _sample_count: u32) -> Self::InputPortType {
        Self {
            pointer: pointer.cast(),
        }
    }

    unsafe fn output_from_raw(pointer: NonNull<()>, _sample_count: u32) -> Self::OutputPortType {
        Self {
            pointer: pointer.cast(),
        }
    }
}