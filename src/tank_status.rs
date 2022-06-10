use wasm_bindgen::prelude::*;
pub const GRID_DIMMENSIONS: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq)]
#[wasm_bindgen]
pub enum TankDirection {
    North,
    West,
    South,
    East,
}

#[derive(Copy, Clone, Debug)]
#[wasm_bindgen]
pub struct TankStatus {
    // (x,y)
    pos: (usize, usize),
    direction: TankDirection,
    // 0 - 100
    health: usize,
    // if tank just shot
    shot: bool,
    ammo_small: usize,
    ammo_big: usize,
}

#[wasm_bindgen]
impl TankStatus {
    pub fn set_dir(&mut self, dir: TankDirection) {
        self.direction = dir;
    }
    pub fn get_dir(&self) -> TankDirection {
        self.direction
    }
    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }
    pub fn calc_radar(&self) -> isize {
        let (new_x, new_y) = self.get_pos();
        match self.get_dir() {
            TankDirection::North => new_y,
            TankDirection::West => new_x,
            TankDirection::South => GRID_DIMMENSIONS - new_y - 1,
            TankDirection::East => GRID_DIMMENSIONS - new_x - 1,
        }
        .try_into()
        .unwrap()
    }

    pub fn set_shot(&mut self, shot: bool) {
        self.shot = shot;
    }

    pub fn shot(&self) -> bool {
        self.shot
    }

    pub fn apply_damage(&mut self, damage: usize) -> usize {
        let new_health = self.health.saturating_sub(damage);
        self.health = new_health;
        new_health
    }

    pub fn default() -> Self {
        Self {
            pos: (0, 0),
            direction: TankDirection::North,
            health: 100,
            shot: false,
            ammo_small: 10000,
            ammo_big: 100,
        }
    }
}

pub trait Position {
    fn get_pos(&self) -> (usize, usize);
}

impl Position for TankStatus {
    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
}
