use std::f32;

use vec2::Vec2;
use line::LineSegment;
use shape::{CollisionShape, AABB, Circle};

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

    pub fn shape_intersection(&self, shape: &CollisionShape) -> Option<Vec2> {
        match shape {
            &CollisionShape::AABB(ref aabb) => self.box_intersection(aabb),
            &CollisionShape::Circle(ref circle) => self.circle_intersection(circle)
        }
    }

    pub fn box_intersection(&self, aabb: &AABB) -> Option<Vec2> {
        
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

        //println!("Ray: {:?}, tmin: {}, tmax: {}", self, tmin, tmax);
        if tmax >= tmin && tmin > 0.0 {
            Some(self.origin + (tmin * self.direction))
        } else if tmin < 0.0 && tmax > 0.0 {
            // from inside rect
            Some(self.origin + (tmax * self.direction))
        } else {
            None
        }
    }

    pub fn circle_intersection(&self, circle: &Circle) -> Option<Vec2> {
        let d = self.direction;
        let f = self.origin - circle.position;

        let a = d.dot_product(d);
        let b = 2.0 * f.dot_product(d);
        let c = f.dot_product(f) - circle.radius * circle.radius;

        let mut discriminant = b * b - 4.0 * a * c;
        //println!("discriminant: {}", discriminant);
        if discriminant < 0.0 {
            None
        } else {
            discriminant = discriminant.sqrt();

            let t1 = (-b - discriminant) / (2.0 * a);
            let t2 = (-b + discriminant) / (2.0 * a);

            let tmin = if t1 < 0.0 {
                // from inside circle
                t2
            } else {
                t1.min(t2)
            };
            //println!("ray: {:?}, tmin: {}, t1: {}, t2: {}", self, tmin, t1, t2);
            if tmin >= 0.0 {
                Some(self.origin + (tmin * self.direction))
            } else {
                None
            }
        }
    }

}
