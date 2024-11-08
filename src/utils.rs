use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{window, Performance};


pub fn js_random() -> f64 {
    js_sys::Math::random()
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
