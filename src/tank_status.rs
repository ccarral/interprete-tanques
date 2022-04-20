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
    pub fn set_dir(&mut self, dir: TankDirection) {
        self.direction = dir;
    }
    pub fn get_dir(&self) -> TankDirection {
        self.direction
    }
}
