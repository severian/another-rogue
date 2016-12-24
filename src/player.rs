use vec2;
use vec2::Vec2;
use bullet::BulletType;

const BOOM_CHARGE_TIME: u32 = 1000;

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub looking_at: Vec2,
    pub gun_is_charging: bool,
    pub gun_charge_start_time: u32
}

impl Player {
    pub fn new() -> Player {
        Player { looking_at: vec2::ORIGIN, gun_is_charging: false, gun_charge_start_time: 0 }
    }

    pub fn start_gun_charging(&mut self, now: u32) {
        self.gun_is_charging = true;
        self.gun_charge_start_time = now;
    }

    pub fn fire_gun(&mut self) {
        self.gun_is_charging = false;
        self.gun_charge_start_time = 0;
    }

    pub fn gun_state(&self, now: u32) -> Option<GunState> {
        if !self.gun_is_charging {
            None
        } else if now - self.gun_charge_start_time < BOOM_CHARGE_TIME {
            Some(GunState::PewPew { charge: ((now - self.gun_charge_start_time) as f32 / BOOM_CHARGE_TIME as f32).min(1.0) })
        } else {
            Some(GunState::Boom { charge: ((now - self.gun_charge_start_time - BOOM_CHARGE_TIME) as f32 / 250.0).min(1.0) })
        }
    }

    pub fn bullet_type(&self, now: u32) -> Option<BulletType> {
        match self.gun_state(now) {
            Some(GunState::PewPew {..}) => Some(BulletType::PewPew),
            Some(GunState::Boom {..}) => Some(BulletType::Boom),
            None => None
        }
    }
}

pub enum GunState {
    PewPew { charge: f32 },
    Boom { charge: f32 }
}


