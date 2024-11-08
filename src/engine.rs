use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, KeyboardEvent};
use std::rc::Rc;
use std::cell::RefCell;
use crate::game::MAP;
use crate::game::MAP_HEIGHT;
use crate::game::MAP_WIDTH;
use crate::renderer::Renderer;
use crate::game::Player;
use crate::raycasting::render_scene;

pub struct Engine {
    player: Player,
    renderer: Renderer,
    keys: Vec<bool>,
}

impl Engine {
    pub fn new() -> Rc<RefCell<Self>> {
        let window = window().expect("No global `window` exists");
        let document = window.document().expect("No document on window");
        let canvas = document
            .get_element_by_id("game-canvas")
            .expect("Canvas element not found")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Failed to convert to HtmlCanvasElement");

        let mut renderer = Renderer::new(canvas.clone());

        let texture_ids = ["texture1", "texture2", "texture3"];
        for texture_id in texture_ids.iter() {
            renderer.load_texture(texture_id);
        }

        let player = Player::new();

        let engine = Rc::new(RefCell::new(Engine {
            player,
            renderer,
            keys: vec![false; 256], // For keyboard input
        }));

        Engine::setup_input(engine.clone());

        engine
    }

    pub fn update(&mut self) {
        // Player movement controls
        if self.keys[b'W' as usize] {
            self.player.move_forward(0.1);
        }
        if self.keys[b'S' as usize] {
            self.player.move_backward(0.1);
        }
        if self.keys[b'A' as usize] {
            self.player.turn_left(0.05);
        }
        if self.keys[b'D' as usize] {
            self.player.turn_right(0.05);
        }
    }

    pub fn render(&mut self) {
        self.renderer.clear();
        render_scene(&self.player, &mut self.renderer);

        self.renderer.draw_minimap(
            &MAP,               // Pass the updated MAP containing different wall types
            MAP_WIDTH,
            MAP_HEIGHT,
            self.player.x,
            self.player.y,
            self.player.direction,
        );
    }

    fn setup_input(engine: Rc<RefCell<Self>>) {
        // Clone `engine` for use in the `keydown` closure
        let engine_keydown = Rc::clone(&engine);
        let on_keydown = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let mut engine = engine_keydown.borrow_mut();
            let key_code = event.key_code() as usize;
            if key_code < engine.keys.len() {
                engine.keys[key_code] = true;
            }
        }) as Box<dyn FnMut(_)>);
    
        // Clone `engine` for use in the `keyup` closure
        let engine_keyup = Rc::clone(&engine);
        let on_keyup = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let mut engine = engine_keyup.borrow_mut();
            let key_code = event.key_code() as usize;
            if key_code < engine.keys.len() {
                engine.keys[key_code] = false;
            }
        }) as Box<dyn FnMut(_)>);
    
        let window = window().expect("No global `window` exists");
    
        // Attach event listeners to the window
        window
            .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
            .unwrap();
        window
            .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
            .unwrap();
    
        // Prevent Rust from deallocating the closures
        on_keydown.forget();
        on_keyup.forget();
    }
}

static mut ENGINE: Option<Rc<RefCell<Engine>>> = None;

#[wasm_bindgen]
pub fn run() {
    unsafe {
        ENGINE = Some(Engine::new());
        let engine = ENGINE.as_ref().unwrap();
        Engine::start(Rc::clone(engine));
    }
}

impl Engine {
    pub fn start(engine: Rc<RefCell<Self>>) {
        let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut engine = engine.borrow_mut();
            engine.update();
            engine.render();

            // Schedule the next frame
            window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .expect("Failed to schedule next frame");
        }) as Box<dyn FnMut()>));

        window()
            .unwrap()
            .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("Failed to start game loop");
    }
}
