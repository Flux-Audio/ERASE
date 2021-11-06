use std::sync::Mutex;
use vst::util::AtomicFloat;

use crate::parameter_manager::{Parameter, ParameterManager};

// VST_LAB calls this to determine which parameters are displayed in the plugin
// window (no GUI) and which parameters the plugin editor can read and write to
// (with GUI)
pub fn create_parameters() -> ParameterManager {
    
    let parameters = vec![
        Parameter {
            display_name: Mutex::new("erase".to_string()),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("hardness".to_string()),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("post gain".to_string()),
            raw: AtomicFloat::new(0.5),
            value_format: Box::new(|x| { format!("{:.2} dB", x*24.0 - 12.0) }),
            ..Default::default()
        },
        Parameter {
            display_name: Mutex::new("inv/dry/wet".to_string()),
            raw: AtomicFloat::new(1.0),
            ..Default::default()
        },
    ];
    ParameterManager::from_vec(parameters)
}