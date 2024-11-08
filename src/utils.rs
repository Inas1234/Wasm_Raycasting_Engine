use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

pub fn js_random() -> f64 {
    js_sys::Math::random()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (js_random() * (max - min))
}
