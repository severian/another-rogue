use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};


pub const ORIGIN: Vec2 = Vec2 { x: 0.0, y: 0.0 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: x, y: y }
    }
    
    pub fn from_ints(x: i32, y: i32) -> Vec2 {
        Vec2::new(x as f32, y as f32)
    }

    pub fn dot_product(&self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn distance(&self, other: Vec2) -> f32 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec2 {
        let m = self.magnitude();
        Vec2::new(self.x / m, self.y / m)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = Vec2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: rhs.x * self, y: rhs.y * self }
    }
}

