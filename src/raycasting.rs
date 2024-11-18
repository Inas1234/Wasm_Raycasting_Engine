use crate::game::{MAP, MAP_HEIGHT, MAP_WIDTH, Player};
use crate::renderer::Renderer;
use crate::sprites::Sprite;

pub struct Ray {
    pub distance: f64,
    pub texture_coord: f64,
    pub vertical_hit: bool,
    pub hit: bool,
    pub texture_id: u8
}

impl Ray {
    pub fn new(distance: f64, texture_coord: f64, vertical_hit: bool, hit: bool, texture_id: u8) -> Self {
        Ray {
            distance,
            texture_coord,
            vertical_hit,
            hit,
            texture_id
        }
    }
}

pub fn cast_ray(player: &Player, angle: f64, cos_angle: f64, sin_angle: f64) -> Ray {
    let step_x = if cos_angle > 0.0 { 1 } else { -1 };
    let step_y = if sin_angle > 0.0 { 1 } else { -1 };

    let delta_dist_x = (1.0 / cos_angle).abs();
    let delta_dist_y = (1.0 / sin_angle).abs();

    let mut side_dist_x = if cos_angle > 0.0 {
        (player.x.floor() + 1.0 - player.x) * delta_dist_x
    } else {
        (player.x - player.x.floor()) * delta_dist_x
    };

    let mut side_dist_y = if sin_angle > 0.0 {
        (player.y.floor() + 1.0 - player.y) * delta_dist_y
    } else {
        (player.y - player.y.floor()) * delta_dist_y
    };

    let mut map_x = player.x as i32;
    let mut map_y = player.y as i32;

    let mut hit = false;
    let mut vertical_hit = false;
    let mut texture_id = 0;

    // Cast rays and minimize calculations
    while !hit {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            vertical_hit = true;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            vertical_hit = false;
        }

        if map_x >= 0 && map_y >= 0 && (map_x as usize) < MAP_WIDTH && (map_y as usize) < MAP_HEIGHT {
            let map_index = (map_y as usize) * MAP_WIDTH + (map_x as usize);
            texture_id = MAP[map_index];

            if texture_id > 0 {
                hit = true;
                break;
            }
        }
    }

    let distance = if vertical_hit {
        (map_x as f64 - player.x + (1.0 - step_x as f64) / 2.0) / cos_angle
    } else {
        (map_y as f64 - player.y + (1.0 - step_y as f64) / 2.0) / sin_angle
    };

    let texture_coord = if vertical_hit {
        (player.y + distance * sin_angle).fract()
    } else {
        (player.x + distance * cos_angle).fract()
    };

    Ray {
        distance,
        texture_coord,
        vertical_hit,
        hit,
        texture_id,
    }
}


pub fn render_scene(player: &Player, renderer: &mut Renderer, sprites: &mut Vec<Sprite>) {
    // Clear the framebuffer before drawing
    renderer.clear_framebuffer();

    let num_rays = renderer.screen_width / 6;
    let screen_width = renderer.screen_width as f64;
    let screen_height = renderer.screen_height as f64;
    let half_screen_height = screen_height / 2.0;

    // Render ceiling and floor first
    for y in 0..renderer.screen_height {
        let is_ceiling = y < (renderer.screen_height / 2);
        let color = if is_ceiling {
            (50, 50, 50)
        } else {
            (100, 100, 100)
        };

        for x in 0..renderer.screen_width {
            let index = (y * renderer.screen_width + x) * 4;
            renderer.framebuffer[index] = color.0;
            renderer.framebuffer[index + 1] = color.1;
            renderer.framebuffer[index + 2] = color.2;
            renderer.framebuffer[index + 3] = 255;
        }
    }

    // Render walls
    for x in 0..num_rays {
        let angle = player.direction - player.fov / 2.0 + (x as f64 / num_rays as f64) * player.fov;
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();

        let ray = cast_ray(player, angle, cos_angle, sin_angle);

        let line_height = (screen_height / ray.distance) as i32;
        let draw_start = (-line_height / 2 + half_screen_height as i32).max(0);
        let draw_end = (line_height / 2 + half_screen_height as i32).min(screen_height as i32 - 1);

        let tex_x = if ray.vertical_hit {
            (player.y + ray.distance * sin_angle).fract() * renderer.texture_width as f64
        } else {
            (player.x + ray.distance * cos_angle).fract() * renderer.texture_width as f64
        } as usize;

        for y in draw_start..draw_end {
            let d = y * 256 - (screen_height as i32 * 128) + line_height * 128;
            let tex_y = ((d * renderer.texture_height as i32) / line_height) / 256;

            let (r, g, b) = renderer.get_texture_color_rgb(ray.texture_id as usize - 1, tex_x, tex_y as usize);
            renderer.draw_rect(x as f64 * 6.0, y as f64, 6.0, 1.0, Some((r, g, b)));
        }
    }
    let dir_x = player.direction.cos();
    let dir_y = player.direction.sin();
    let fov_factor = (player.fov / 2.0).tan();
    let plane_x = -dir_y * fov_factor;
    let plane_y = dir_x * fov_factor;


    // Sort sprites by distance from player
    for sprite in sprites.iter_mut() {
        let dx = sprite.x - player.x;
        let dy = sprite.y - player.y;
        sprite.distance = (dx * dx + dy * dy).sqrt();
    }
    sprites.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());

    // Render sprites
    for sprite in sprites.iter() {
        // Translate sprite position to relative to camera
        let sprite_x = sprite.x - player.x;
        let sprite_y = sprite.y - player.y;

        // Inverse determinant for correct matrix multiplication
        let inv_det = 1.0 / (plane_x * dir_y - dir_x * plane_y);

        // Transform sprite with the inverse camera matrix
        let transform_x = inv_det * (dir_y * sprite_x - dir_x * sprite_y);
        let transform_y = inv_det * (-plane_y * sprite_x + plane_x * sprite_y);

        // Avoid rendering sprites behind the camera
        if transform_y <= 0.0 {
            continue;
        }

        // Project the sprite onto the screen
        let sprite_screen_x = ((screen_width / 2.0) * (1.0 + transform_x / transform_y)) as i32;

        // Calculate sprite dimensions on screen
        let sprite_height = (screen_height / transform_y).abs() as i32;
        let sprite_width = sprite_height;

        // Calculate drawing boundaries
        let draw_start_y = (-sprite_height / 2 + half_screen_height as i32).max(0);
        let draw_end_y = (sprite_height / 2 + half_screen_height as i32).min(screen_height as i32 - 1);
        let draw_start_x = (-sprite_width / 2 + sprite_screen_x).max(0);
        let draw_end_x = (sprite_width / 2 + sprite_screen_x).min(renderer.screen_width as i32 - 1);

        // Get the texture for the sprite
        let texture_index = sprite.texture_id as usize - 1;
        if texture_index >= renderer.textures.len() {
            continue;
        }
        let texture = &renderer.textures[texture_index];

        // Loop through every vertical stripe of the sprite on screen
// Inside the sprite rendering loop
        for stripe in draw_start_x..draw_end_x {
            if stripe < 0 || stripe >= renderer.screen_width as i32 {
                continue;
            }

            let tex_x = ((stripe - (-sprite_width / 2 + sprite_screen_x)) * renderer.texture_width as i32) / sprite_width;

            for y in draw_start_y..draw_end_y {
                if y < 0 || y >= renderer.screen_height as i32 {
                    continue;
                }

                let d = y * 256 - (screen_height as i32) * 128 + sprite_height * 128;
                let tex_y = ((d * renderer.texture_height as i32) / sprite_height) / 256;

                let texture = &renderer.textures[texture_index];
                let tex_index = ((tex_y * renderer.texture_width as i32 + tex_x) * 4) as usize;

                if tex_index + 3 >= texture.len() {
                    continue;
                }

                let tex_r = texture[tex_index] as f32;
                let tex_g = texture[tex_index + 1] as f32;
                let tex_b = texture[tex_index + 2] as f32;
                let tex_a = texture[tex_index + 3] as f32 / 255.0; // Normalize alpha to [0,1]

                // Skip fully transparent pixels
                if tex_a <= 0.0 {
                    continue;
                }

                let screen_index = (y as usize * renderer.screen_width + stripe as usize) * 4;

                let bg_r = renderer.framebuffer[screen_index] as f32;
                let bg_g = renderer.framebuffer[screen_index + 1] as f32;
                let bg_b = renderer.framebuffer[screen_index + 2] as f32;
                let bg_a = renderer.framebuffer[screen_index + 3] as f32 / 255.0;

                let out_a = tex_a + bg_a * (1.0 - tex_a);
                if out_a > 0.0 {
                    let out_r = (tex_r * tex_a + bg_r * bg_a * (1.0 - tex_a)) / out_a;
                    let out_g = (tex_g * tex_a + bg_g * bg_a * (1.0 - tex_a)) / out_a;
                    let out_b = (tex_b * tex_a + bg_b * bg_a * (1.0 - tex_a)) / out_a;
    
                    renderer.framebuffer[screen_index] = out_r as u8;
                    renderer.framebuffer[screen_index + 1] = out_g as u8;
                    renderer.framebuffer[screen_index + 2] = out_b as u8;
                    renderer.framebuffer[screen_index + 3] = (out_a * 255.0) as u8;
                } else {
                    renderer.framebuffer[screen_index + 3] = 0;
                }
            }
        }
    }

    renderer.flush();
}
