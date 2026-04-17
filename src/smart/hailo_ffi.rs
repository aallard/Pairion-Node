//! HailoRT FFI wrapper.
//!
//! Thin Rust wrapper over HailoRT's C API for the AI HAT+ 2 NPU.
//! No other module calls HailoRT directly (CONVENTIONS §2.7).
//!
//! In M0, all functions are `unimplemented!()`. Real FFI bindings arrive at M7.

use crate::capabilities::manifest::NodeCapabilities;

/// Handle to an initialized Hailo device.
pub struct HailoDevice {
    _private: (),
}

/// Handle to a loaded model on the Hailo device.
pub struct HailoModel {
    _private: (),
}

/// Initialize the Hailo device.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0. Real implementation at M7.
pub fn init_device(capabilities: &NodeCapabilities) -> HailoDevice {
    assert!(
        capabilities.ai_accelerator != crate::capabilities::manifest::AiAccelerator::None,
        "Hailo init called on Node without AI accelerator"
    );
    unimplemented!("HailoRT FFI not implemented until M7")
}

/// Shut down the Hailo device.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0.
pub fn shutdown_device(_device: HailoDevice) {
    unimplemented!("HailoRT FFI not implemented until M7")
}

/// Load a model onto the Hailo device.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0.
pub fn load_model(_device: &HailoDevice, _path: &str) -> HailoModel {
    unimplemented!("HailoRT FFI not implemented until M7")
}

/// Run inference on a loaded model.
///
/// # Panics
///
/// Panics with `unimplemented!()` in M0.
pub fn infer(_model: &HailoModel, _input: &[u8]) -> Vec<u8> {
    unimplemented!("HailoRT FFI not implemented until M7")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::manifest::{AiAccelerator, NodeCapabilities};

    #[test]
    #[should_panic(expected = "Hailo init called on Node without AI accelerator")]
    fn init_device_rejects_dumb_tier() {
        let caps = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: false,
            local_llm_small: false,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::None,
            dedicated_npu_ram_gb: 0,
        };
        let _device = init_device(&caps);
    }

    #[test]
    #[should_panic(expected = "HailoRT FFI not implemented until M7")]
    fn init_device_unimplemented_for_smart() {
        let caps = NodeCapabilities {
            audio_in: true,
            audio_out: true,
            local_wake_word: true,
            local_vad: true,
            local_stt: true,
            local_llm_small: true,
            local_tts_cache: true,
            ai_accelerator: AiAccelerator::Hailo10h,
            dedicated_npu_ram_gb: 8,
        };
        let _device = init_device(&caps);
    }
}
