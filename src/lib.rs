use wasm_bindgen::prelude::*;

mod renderer;
mod engine;
#[macro_use]
mod utils;
mod game;
mod raycasting;
mod sprites;


#[wasm_bindgen]
pub fn start(){
    engine::run();
}
