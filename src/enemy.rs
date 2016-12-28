use bullet::{Bullet, BulletType};
use entity::{Entity, Physics};
use shape::Shape;

const HEALTH_REGEN_PER_MS: f32 = 12.0 / 1000.0;

#[derive(Debug, Copy, Clone)]
pub struct Enemy {
    pub inner_radius: f32,
    pub shield_health: f32
}

impl Enemy {
    pub fn new(inner_radius: f32) -> Enemy {
        Enemy { inner_radius: inner_radius, shield_health: 100.0 }
    }

    pub fn has_shield(&self) -> bool {
        self.shield_health > 0.0
    }

    pub fn take_hit(&mut self, bullet: &Entity) {
        if self.has_shield() && bullet.bullet().bullet_type == BulletType::Boom {
            self.shield_health -= 75.0;
        }
    }

    pub fn update(&mut self, physics: &mut Physics, time_delta: u32) {
        self.shield_health = (self.shield_health + HEALTH_REGEN_PER_MS * time_delta as f32).min(100.0);

        let radius = if self.has_shield() {
            self.inner_radius + 10.0
        } else {
            self.inner_radius
        };

        physics.shape = Shape::Circle { radius: radius }
    }

}

