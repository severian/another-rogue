use std::f32;

use bullet::{BulletType};
use entity::{Entity, Physics};

const HEALTH_REGEN_PER_MS: f32 = 0.12 / 1000.0;

const SHIELD_SLICES: u32 = 12;

#[derive(Debug, Copy, Clone)]
pub struct Enemy {
    pub inner_radius: f32,
    pub shield_health: [f32; SHIELD_SLICES as usize]
}

impl Enemy {
    pub fn new(inner_radius: f32) -> Enemy {
        Enemy {
            inner_radius: inner_radius,
            shield_health: [1.0; SHIELD_SLICES as usize]
        }
    }

    pub fn take_hit(&mut self, physics: &Physics, bullet: &Entity) {
        if bullet.bullet().bullet_type == BulletType::Boom {

            let pos = bullet.physics.position - physics.position;
            let mut angle = pos.y.atan2(pos.x);
            if angle < 0.0 {
                angle += 2.0 * f32::consts::PI;
            }

            let shield_slice = ((angle / (2.0 * f32::consts::PI)) * SHIELD_SLICES as f32).round() as usize;

            if self.shield_health[shield_slice] > 0.0 {
                self.shield_health[shield_slice] -= 0.75;
            }

            //println!("Angle: {}, shield slice: {}", angle, shield_slice);
        }
    }

    pub fn update(&mut self, time_delta: u32) {
        for shield_health in &mut self.shield_health {
            *shield_health = (*shield_health + HEALTH_REGEN_PER_MS * time_delta as f32).min(1.0);
        }

    }

}

