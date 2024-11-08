use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Renderer { context }
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, 800.0, 600.0);
    }

    pub fn draw_circle(&self, x: f64, y: f64, radius: f64, color: &str, opacity: f64) {
        self.context.begin_path();
        self.context
            .arc(x, y, radius, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        self.context.set_fill_style_str(color);
        self.context.set_global_alpha(opacity);
        self.context.fill();
        self.context.set_global_alpha(1.0); // Reset opacity
    }

    pub fn draw_line(&self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        self.context.set_fill_style_str(color);
        self.context.fill_rect(x, y, width, height);
    }
}
