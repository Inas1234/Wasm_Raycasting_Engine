use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub struct Renderer {
    context: CanvasRenderingContext2d,
    texture_data: Option<Vec<u8>>,
    pub texture_width: usize,
    pub texture_height: usize
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Renderer { context, texture_data: None, texture_width: 0, texture_height: 0 }
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

    pub fn draw_minimap(
        &self,
        map: &[u8],
        map_width: usize,
        map_height: usize,
        player_x: f64,
        player_y: f64,
        player_dir: f64,
    ) {
        let scale = 10.0; // Scale factor for the minimap size
        let offset_x = 20.0;
        let offset_y = 20.0;

        // Draw the map
        for y in 0..map_height {
            for x in 0..map_width {
                let color = if map[y * map_width + x] == 1 {
                    "black"
                } else {
                    "lightgray"
                };
                self.draw_line(
                    offset_x + (x as f64) * scale,
                    offset_y + (y as f64) * scale,
                    scale,
                    scale,
                    color,
                );
            }
        }

        // Draw the player
        let player_map_x = offset_x + player_x * scale;
        let player_map_y = offset_y + player_y * scale;
        self.draw_circle(player_map_x, player_map_y, 4.0, "blue", 100.0);

        // Draw player's field of view
        let fov_length = 15.0; // Length of the FOV line
        let fov_x = player_map_x + fov_length * player_dir.cos();
        let fov_y = player_map_y + fov_length * player_dir.sin();
        self.context.begin_path();
        self.context.move_to(player_map_x, player_map_y);
        self.context.line_to(fov_x, fov_y);
        self.context.set_fill_style_str("red"); 
        self.context.stroke();
    }

    pub fn load_texture(&mut self, image_id: &str){
        let document = web_sys::window().unwrap().document().unwrap();
        let image_element = document
            .get_element_by_id(image_id)
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(image_element.width());
        canvas.set_height(image_element.height());

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context
            .draw_image_with_html_image_element(&image_element, 0.0, 0.0)
            .unwrap();

        let image_data = context
            .get_image_data(0.0, 0.0, image_element.width() as f64, image_element.height() as f64)
            .unwrap()
            .data()
            .to_vec();

        self.texture_data = Some(image_data);
        self.texture_width = image_element.width() as usize;
        self.texture_height = image_element.height() as usize;

    }

    pub fn get_texture_color(&self, tex_x: usize, tex_y: usize) -> String {
        if let Some(ref texture_data) = self.texture_data {
            let index = ((tex_y * self.texture_width + tex_x) * 4) as usize;
            if index + 3 < texture_data.len() {
                let r = texture_data[index];
                let g = texture_data[index + 1];
                let b = texture_data[index + 2];
                return format!("rgb({}, {}, {})", r, g, b);
            }
        }
        "black".to_string()
    }
    
}
