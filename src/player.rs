use bullet::BulletType;

const BOOM_CHARGE_TIME: u32 = 1000;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub gun_is_charging: bool,
    pub gun_charge_start_time: u32
}

impl Player {
    pub fn new() -> Player {
        Player { gun_is_charging: false, gun_charge_start_time: 0 }
    }

    pub fn start_gun_charging(&mut self, now: u32) {
        self.gun_is_charging = true;
        self.gun_charge_start_time = now;
    }

    pub fn fire_gun(&mut self) {
        self.gun_is_charging = false;
        self.gun_charge_start_time = 0;
    }

    pub fn gun_state(&self, now: u32) -> GunState {
        if !self.gun_is_charging {
            GunState::Off
        } else if now - self.gun_charge_start_time < BOOM_CHARGE_TIME {
            GunState::PewPew
        } else {
            GunState::Boom
        }
    }

    pub fn bullet_type(&self, now: u32) -> Option<BulletType> {
        match self.gun_state(now) {
            GunState::PewPew => Some(BulletType::PewPew),
            GunState::Boom => Some(BulletType::Boom),
            GunState::Off => None
        }
    }
}

pub enum GunState {
    Off,
    PewPew,
    Boom
}


