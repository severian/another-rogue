use sdl2::pixels::Color;

#[derive(Debug, Copy, Clone)]
pub enum BulletType {
    PewPew,
    Boom
}

#[derive(Debug, Copy, Clone)]
pub struct Bullet {
    bullet_type: BulletType
}

impl Bullet {
    pub fn new(bullet_type: BulletType) -> Bullet {
        Bullet { bullet_type: bullet_type }
    }

    pub fn color(&self) -> Color {
        match self.bullet_type {
            BulletType::PewPew => Color::RGB(255, 255, 0),
            BulletType::Boom => Color::RGB(255, 0, 0)
        }
    }
}



