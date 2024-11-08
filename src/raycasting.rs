use crate::game::{MAP, MAP_HEIGHT, MAP_WIDTH, Player};
use crate::renderer::Renderer;


pub struct Ray {
    pub distance: f64,
    pub hit: bool
}

pub fn cast_ray(player: &Player, angle: f64) -> Ray {
    let mut ray_x = player.x;
    let mut ray_y = player.y;

    let sin_angle = angle.sin();
    let cos_angle = angle.cos();

    let mut distance = 0.0;

    loop {
        distance += 0.01;
        ray_x = player.x + cos_angle * distance;
        ray_y = player.y + sin_angle * distance;

        let map_x = ray_x as usize;
        let map_y = ray_y as usize;

        if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT || MAP[map_y * MAP_WIDTH + map_x] == 1 {
            return Ray { distance, hit: true };
        }

        if distance > 20.0 {
            break;
        }
    } 

    Ray{distance, hit: false}
}

pub fn render_scene(player: &Player, renderer: &mut Renderer) {
    let num_rays = 120;
    let half_fov = player.fov / 2.0;

    for i in 0..num_rays {
        let angle = player.direction - half_fov + (i as f64 / num_rays as f64) * player.fov;
        let ray = cast_ray(player, angle);

        let line_height = (1.0 / ray.distance) * 400.0;
        let line_height = line_height.min(600.0);

        let color = if ray.hit { "rgb(200, 0, 0)" } else { "rgb(0, 200, 0)" };
        renderer.draw_line(i as f64 * 6.0, 300.0 - line_height / 2.0, 6.0, line_height, color);
    }
}
