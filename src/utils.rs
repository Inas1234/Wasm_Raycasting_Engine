use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{window, Performance, console};
use js_sys::Math;

/// Generate a random floating-point number between 0 and 1.
pub fn js_random() -> f64 {
    Math::random()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (js_random() * (max - min))
}

pub fn get_performance() -> Performance {
    window()
        .expect("No global `window` exists")
        .performance()
        .expect("Performance should be available")
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        // Use the fully qualified path to the console log function
        web_sys::console::log_1(&format_args!($($t)*).to_string().into());
    };
}
