use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

pub struct Renderer {
    pub context: CanvasRenderingContext2d,
    framebuffer: Vec<u8>, // Store the entire screen in a buffer
    textures: Vec<Vec<u8>>,
    pub texture_width: usize,
    pub texture_height: usize,
    pub screen_width: usize,
    pub screen_height: usize,

}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
    

            let screen_width = canvas.width() as usize;
            let screen_height = canvas.height() as usize;
    
    
            Renderer {
                context,
                framebuffer: vec![0; (screen_width * screen_height * 4) as usize], // RGBA buffer
                textures: Vec::new(),
                texture_width: 64,
                texture_height: 64,
                screen_height: screen_height,
                screen_width: screen_width
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
    
    pub fn load_texture(&mut self, texture_id: &str) {
        let document = web_sys::window().unwrap().document().unwrap();
        let img_element = document
            .get_element_by_id(texture_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_width(self.texture_width as u32);
        canvas.set_height(self.texture_height as u32);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context
            .draw_image_with_html_image_element(&img_element, 0.0, 0.0)
            .unwrap();

        let image_data = context
            .get_image_data(0.0, 0.0, self.texture_width as f64, self.texture_height as f64)
            .unwrap();

        let texture_data = image_data.data().to_vec();
        self.textures.push(texture_data);
    }


    pub fn get_texture_color(&self, texture_index: usize, tex_x: usize, tex_y: usize) -> String {
        if texture_index >= self.textures.len() {
            return "black".to_string();
        }

        let texture = &self.textures[texture_index];
        let tex_width = self.texture_width;
        let tex_height = self.texture_height;

        let index = (tex_y * tex_width + tex_x) * 4;

        if index + 3 >= texture.len() {
            return "black".to_string();
        }

        let r = texture[index];
        let g = texture[index + 1];
        let b = texture[index + 2];
        format!("rgb({}, {}, {})", r, g, b)
    }

    pub fn draw_text(&self, x: f64, y: f64, text: &str) {
        self.context.set_fill_style_str("white");
        self.context.set_font("16px Arial");
        self.context.fill_text(text, x, y).unwrap();
    }

    pub fn flush(&self) {
        
        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&self.framebuffer),
            self.screen_width as u32,
            self.screen_height as u32,
        )
        .unwrap();
        self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }

    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64, color: &str) {
        let (r, g, b) = match color {
            "black" => (0, 0, 0),
            "lightgray" => (211, 211, 211),
            "darkgray" => (169, 169, 169),
            "lightblue" => (173, 216, 230),
            _ => (255, 255, 255),
        };

        let start_x = x as u32;
        let start_y = y as u32;
        let end_x = (x + width) as u32;
        let end_y = (y + height) as u32;

        for px in start_x..end_x {
            for py in start_y..end_y {
                let index = ((py * self.screen_width as u32 + px) * 4) as usize;
                self.framebuffer[index] = r;
                self.framebuffer[index + 1] = g;
                self.framebuffer[index + 2] = b;
                self.framebuffer[index + 3] = 255; // Alpha channel
            }
        }
    }



    
}