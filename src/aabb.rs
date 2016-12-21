use vec2::Vec2;
use line::LineSegment;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2
}

impl AABB {
    pub fn extent_x(self) -> f32 {
        (self.max.x - self.min.x) / 2.0
    }

    pub fn extent_y(self) -> f32 {
        (self.max.y - self.min.y) / 2.0
    }

    pub fn line_segments(self) -> [LineSegment; 4] {
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

