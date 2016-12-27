
#[derive(Debug, Copy, Clone)]
pub struct Enemy {
    pub inner_radius: f32
}

impl Enemy {
    pub fn new(inner_radius: f32) -> Enemy {
        Enemy { inner_radius: inner_radius }
    }
}

