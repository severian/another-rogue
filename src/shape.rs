use vec2::Vec2;
use line::LineSegment;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2
}

impl AABB {

    pub fn new(min: Vec2, max: Vec2) -> AABB {
        AABB { min: min, max: max }
    }

    pub fn half_extent(&self) -> Vec2 {
        Vec2::new(self.max.x - self.min.x, self.max.y - self.min.y) / 2.0
    }

    pub fn position(&self) -> Vec2 {
        self.min + self.half_extent()
    }

    pub fn line_segments(&self) -> [LineSegment; 4] {
        let top_right = Vec2::new(self.max.x, self.min.y);
        let bottom_left = Vec2::new(self.min.x, self.max.y);

        [
            LineSegment::new(self.min, top_right),
            LineSegment::new(top_right, self.max),
            LineSegment::new(self.max, bottom_left),
            LineSegment::new(bottom_left, self.min)
        ]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub position: Vec2,
    pub radius: f32
}

impl Circle {
    pub fn new(position: Vec2, radius: f32) -> Circle {
        Circle { position: position, radius: radius }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Rect { extent: Vec2 },
    Circle { radius: f32 }
}

#[derive(Debug, Copy, Clone)]
pub enum CollisionShape {
    AABB(AABB),
    Circle(Circle)
}

