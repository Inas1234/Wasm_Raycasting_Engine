use wasm_bindgen::prelude::*;

mod renderer;
mod engine;
mod utils;
mod game;
mod raycasting;


#[wasm_bindgen]
pub fn start(){
    engine::run();
}
