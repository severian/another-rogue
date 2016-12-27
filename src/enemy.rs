use bullet::Bullet;
use entity::{Entity, Physics};
use shape::Shape;

#[derive(Debug, Copy, Clone)]
pub struct Enemy {
    pub inner_radius: f32,
    pub shield_health: i32
}

impl Enemy {
    pub fn new(inner_radius: f32) -> Enemy {
        Enemy { inner_radius: inner_radius, shield_health: 100 }
    }

    pub fn has_shield(&self) -> bool {
        self.shield_health > 0
    }

    pub fn take_hit(&mut self, bullet: &Entity) {
        self.shield_health -= 50;
    }

    pub fn update_physics(&self, physics: &mut Physics) {
        let radius = if self.has_shield() {
            self.inner_radius + 5.0
        } else {
            self.inner_radius
        };

        physics.shape = Shape::Circle { radius: radius }
    }

}

