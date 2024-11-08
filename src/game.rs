pub const MAP_WIDTH: usize = 8;
pub const MAP_HEIGHT: usize = 8;

pub const MAP: [u8; MAP_WIDTH * MAP_HEIGHT] = [
    1, 1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 2, 0, 3, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 1,
    1, 1, 1, 1, 1, 1, 1, 1,
];

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub direction: f64,
    pub fov: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: 3.5,
            y: 3.5,
            direction: 0.0,
            fov: std::f64::consts::PI / 3.0,
        }
    }

    pub fn move_forward(&mut self, distance: f64) {
        let new_x = self.x + distance * self.direction.cos();
        let new_y = self.y + distance * self.direction.sin();

        if !self.is_colliding(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f64) {
        let new_x = self.x - distance * self.direction.cos();
        let new_y = self.y - distance * self.direction.sin();

        if !self.is_colliding(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn turn_left(&mut self, angle: f64) {
        self.direction -= angle;
    }

    pub fn turn_right(&mut self, angle: f64) {
        self.direction += angle;
    }

    pub fn is_colliding(&self, new_x: f64, new_y: f64) -> bool {
        let map_x = new_x as usize;
        let map_y = new_y as usize;

        if map_x >= MAP_WIDTH || map_y >= MAP_HEIGHT{
            return true;
        }

        MAP[map_y * MAP_WIDTH + map_x] == 1
    }
}
