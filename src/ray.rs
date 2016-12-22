use std::f32;

use vec2::Vec2;
use line::LineSegment;
use shape::AABB;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec2,
    pub direction: Vec2
}

impl Ray {

    pub fn new(origin: Vec2, direction: Vec2) -> Ray {
        Ray { origin: origin, direction: direction }
    }

    pub fn from_segment(segment: &LineSegment) -> Ray {
        Ray::new(segment.start, segment.end - segment.start)
    }

    pub fn box_intersection(self, aabb: &AABB) -> Option<Vec2> {
        
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;

        if self.direction.x != 0.0 {
            let tx1 = (aabb.min.x - self.origin.x) / self.direction.x;
            let tx2 = (aabb.max.x - self.origin.x) / self.direction.x;

            tmin = tmin.max(tx1.min(tx2));
            tmax = tmax.min(tx1.max(tx2));
        }

        if self.direction.y != 0.0 {
            let ty1 = (aabb.min.y - self.origin.y) / self.direction.y;
            let ty2 = (aabb.max.y - self.origin.y) / self.direction.y;

            tmin = tmin.max(ty1.min(ty2));
            tmax = tmax.min(ty1.max(ty2));
        }

        if tmax >= tmin && tmin >= 0.0 {
            //println!("Ray: {:?}", self);
            //println!("tmin: {}, tmax: {}", tmin, tmax);
            Some(self.origin + (tmin * self.direction))
        } else {
            None
        }
    }

}
