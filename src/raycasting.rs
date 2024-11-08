use crate::game::{MAP, MAP_HEIGHT, MAP_WIDTH, Player};
use crate::renderer::Renderer;


pub struct Ray {
    pub distance: f64,
    pub texture_coord: f64,
    pub vertical_hit: bool,
    pub hit: bool,
}

impl Ray {
    pub fn new(distance: f64, texture_coord: f64, vertical_hit: bool, hit: bool) -> Self {
        Ray {
            distance,
            texture_coord,
            vertical_hit,
            hit,
        }
    }
}

pub fn cast_ray(player: &Player, angle: f64) -> Ray {
    let sin_angle = angle.sin();
    let cos_angle = angle.cos();

    let step_x = if cos_angle > 0.0 { 1 } else { -1 };
    let step_y = if sin_angle > 0.0 { 1 } else { -1 };

    let delta_dist_x = (1.0 / cos_angle.abs()).abs();
    let delta_dist_y = (1.0 / sin_angle.abs()).abs();

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

        if map_x >= 0 && map_y >= 0 && map_x < MAP_WIDTH as i32 && map_y < MAP_HEIGHT as i32 {
            if MAP[map_y as usize * MAP_WIDTH + map_x as usize] == 1 {
                hit = true;
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
    }
}

pub fn render_scene(player: &Player, renderer: &mut Renderer) {
    let num_rays = 120;
    let half_fov = player.fov / 2.0;
    let screen_height = 600.0;

    for x in 0..num_rays {
        let angle = player.direction - half_fov + (x as f64 / num_rays as f64) * player.fov;
        let ray = cast_ray(player, angle);

        if ray.hit {
            let line_height = (screen_height / ray.distance) as i32;
            let draw_start = (-line_height / 2 + (screen_height as i32) / 2).max(0);
            let draw_end = (line_height / 2 + (screen_height as i32) / 2).min(screen_height as i32 - 1);

            let tex_x = (ray.texture_coord * renderer.texture_width as f64) as usize % renderer.texture_width;

            for y in draw_start..draw_end {
                let d = y * 256 - (screen_height as i32 * 128) + line_height * 128;
                let tex_y = ((d * renderer.texture_height as i32) / line_height) / 256;
                
                let color = renderer.get_texture_color(tex_x, tex_y as usize);
                renderer.draw_line(
                    x as f64 * 6.0,
                    y as f64,
                    6.0,
                    1.0,
                    &color,
                );
            }
        }
    }
}
