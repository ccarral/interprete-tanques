#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TankDirection {
    North,
    West,
    South,
    East,
}

#[derive(Copy, Clone)]
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

impl Default for TankStatus {
    fn default() -> Self {
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

impl TankStatus {
    pub const GRID_DIMMENSIONS: usize = 12;
    pub fn set_dir(&mut self, dir: TankDirection) {
        self.direction = dir;
    }
    pub fn get_dir(&self) -> TankDirection {
        self.direction
    }
    pub fn get_pos(&self) -> (usize, usize) {
        self.pos
    }
    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }
}
