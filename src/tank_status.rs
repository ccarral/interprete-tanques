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
    // (i,j)
    pos: (usize, usize),
    direction: TankDirection,
    // 0 - 100
    health: usize,
    // if tank just shot
    shot: bool,
    // if tank got shot
    got_shot: bool,
    ammo_small: usize,
    ammo_big: usize,
    current_interpreter_line: usize,
}

#[wasm_bindgen]
impl TankStatus {
    pub fn set_dir(&mut self, dir: TankDirection) {
        self.direction = dir;
    }
    pub fn get_dir(&self) -> TankDirection {
        self.direction
    }
    pub fn set_pos(&mut self, i: usize, j: usize) {
        self.pos = (i, j);
    }
    pub fn calc_radar(&self) -> isize {
        let (new_i, new_j) = self.get_pos();
        match self.get_dir() {
            TankDirection::West => new_j,
            TankDirection::North => new_i,
            TankDirection::East => GRID_DIMMENSIONS - new_j - 1,
            TankDirection::South => GRID_DIMMENSIONS - new_i - 1,
        }
        .try_into()
        .unwrap()
    }

    pub fn set_shot(&mut self, shot: bool) {
        self.shot = shot;
    }

    #[wasm_bindgen(getter)]
    pub fn shot(&self) -> bool {
        self.shot
    }

    #[wasm_bindgen(getter)]
    pub fn got_shot(&self) -> bool {
        self.got_shot
    }

    pub fn set_got_shot(&mut self, got_shot: bool) {
        self.got_shot = got_shot;
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
            got_shot: false,
            ammo_small: 10000,
            ammo_big: 100,
            current_interpreter_line: 0,
        }
    }

    #[wasm_bindgen(constructor)]
    pub fn new(i: usize, j: usize) -> Self {
        let mut status = Self::default();
        status.set_pos(i, j);
        status
    }

    #[wasm_bindgen(getter)]
    pub fn current_interpreter_line(&self) -> usize {
        self.current_interpreter_line
    }

    pub fn set_current_interpreter_line(&mut self, line: usize) {
        self.current_interpreter_line = line;
    }

    #[wasm_bindgen(getter)]
    pub fn pos_i(&self) -> usize {
        self.pos.0
    }

    #[wasm_bindgen(getter)]
    pub fn pos_j(&self) -> usize {
        self.pos.1
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
