use std::f32;

use vec2::Vec2;

const INTERSECTION_EPSILON: f32 = 0.00003;

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub a: f32,
    pub b: f32,
    pub c: f32
}

impl Line {
    pub fn from_segment(segment: LineSegment) -> Line {
        let a = segment.end.y - segment.start.y;
        let b = segment.start.x - segment.end.x;
        let c = a * segment.start.x + b * segment.start.y;
        Line { a: a, b: b, c: c }
    }

    pub fn intersection(self, other: Line) -> Option<Vec2> {
        let det = self.a * other.b - other.a * self.b;
        if det != 0.0 {
            let x = (other.b * self.c - self.b * other.c) / det;
            let y = (self.a * other.c - other.a * self.c) / det;

            return Some(Vec2::new(x, y));
        }

        return None;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2
}

impl LineSegment { 
    
    pub fn new(start: Vec2, end: Vec2) -> LineSegment {
        LineSegment { start: start, end: end }
    }

    pub fn has_point(self, point: Vec2) -> bool {
        point.x >= self.start.x.min(self.end.x) - INTERSECTION_EPSILON && point.x <= self.start.x.max(self.end.x) + INTERSECTION_EPSILON
            && point.y >= self.start.y.min(self.end.y) - INTERSECTION_EPSILON && point.y <= self.start.y.max(self.end.y) + INTERSECTION_EPSILON
    }

    pub fn intersection(self, other: LineSegment) -> Option<Vec2> {
        let line1 = Line::from_segment(self);
        let line2 = Line::from_segment(other);
        
        line1.intersection(line2).and_then(|intersection| {
            //println!("");
            //println!("Line intersection: {:?}", intersection);
            if self.has_point(intersection) && other.has_point(intersection) {
                //println!("Segment intersection: {:?}", intersection);
                return Some(intersection);
            } else {
                return None;
            }
        })
    }

}


