use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub struct Renderer {
    context: CanvasRenderingContext2d,
    textures: Vec<Vec<u8>>,
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
    
            let texture_width = 64;
            let texture_height = 64;
    
            Renderer {
                context,
                textures: Vec::new(),
                texture_width,
                texture_height,
            }
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
        map: &[u8],         // Updated to match the type of your MAP (i32)
        map_width: usize,
        map_height: usize,
        player_x: f64,
        player_y: f64,
        player_dir: f64,
    ) {
        let scale = 10.0;
        let offset_x = 20.0;
        let offset_y = 20.0;
    
        // Define colors for different wall textures
        let texture_colors = [
            "black",        // Texture 1
            "brown",        // Texture 2
            "darkgreen",    // Texture 3
            "gray",         // Default for unknown textures
        ];
    
        // Draw the map
        for y in 0..map_height {
            for x in 0..map_width {
                let tile_value = map[y * map_width + x];
                let color = if tile_value > 0 && tile_value <= texture_colors.len() as u8 {
                    texture_colors[(tile_value - 1) as usize] // Adjust for 0-based index
                } else {
                    "lightgray" // Empty space
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
        self.context.set_stroke_style(&"red".into());
        self.context.stroke();
    }
    
    pub fn load_texture(&mut self, image_id: &str) {
        let document = web_sys::window().unwrap().document().unwrap();
        let img = document
            .get_element_by_id(image_id)
            .unwrap()
            .dyn_into::<HtmlImageElement>()
            .unwrap();
    
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(img.width());
        canvas.set_height(img.height());
    
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        context
            .draw_image_with_html_image_element(&img, 0.0, 0.0)
            .unwrap();
    
        let image_data = context
            .get_image_data(0.0, 0.0, img.width() as f64, img.height() as f64)
            .unwrap()
            .data()
            .to_vec();
    
        self.textures.push(image_data);
    }


    pub fn get_texture_color(&self, texture_index: usize, tex_x: usize, tex_y: usize) -> String {
        if let Some(texture_data) = self.textures.get(texture_index) {
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

    pub fn draw_text(&self, x: f64, y: f64, text: &str) {
        self.context.set_fill_style_str("white");
        self.context.set_font("16px Arial");
        self.context.fill_text(text, x, y).unwrap();
    }

    
}