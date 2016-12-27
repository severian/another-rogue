use std::f32;

use vec2::Vec2;
use entity::{Physics, Entity};
use line::LineSegment;
use ray::Ray;
use shape::{AABB, Circle, CollisionShape};


#[derive(Debug, Copy, Clone)]
pub struct Manifold {
    pub penetration: f32,
    pub normal: Vec2
}

impl Manifold {
    pub fn new(penetration: f32, normal: Vec2) -> Manifold {
        Manifold { penetration: penetration, normal: normal }
    }
}

// from https://gamedevelopment.tutsplus.com/tutorials/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331

pub fn collision_manifold(a: &Entity, b: &Entity) -> Option<Manifold> {
    match (a.physics.collision_shape(), b.physics.collision_shape()) {
        (CollisionShape::AABB(abox), CollisionShape::AABB(bbox)) => aabb_aabb_collision_manifold(&abox, &bbox),
        (CollisionShape::Circle(acirc), CollisionShape::Circle(bcirc)) => circle_circle_collision_manifold(&acirc, &bcirc),
        (CollisionShape::AABB(abox), CollisionShape::Circle(bcirc)) => aabb_circle_collision_manifold(&abox, &bcirc),
        (CollisionShape::Circle(acirc), CollisionShape::AABB(bbox)) =>
            aabb_circle_collision_manifold(&bbox, &acirc).as_mut().map(|m| {
                m.normal *= -1.0;
                *m
            })
    }
}

fn aabb_aabb_collision_manifold(a: &AABB, b: &AABB) -> Option<Manifold> {
    let n = b.position() - a.position();

    //println!("Normal: {:?}", n);
    //println!("A box: {:?}", a);
    //println!("B box: {:?}", b);

    let a_extent = a.half_extent();
    let b_extent = b.half_extent();

    let x_overlap = a_extent.x + b_extent.x - n.x.abs();

    if x_overlap > 0.0 {
        let y_overlap = a_extent.y + b_extent.y - n.y.abs();

        if y_overlap > 0.0 {
            if x_overlap < y_overlap {
                let normal = if n.x < 0.0 { Vec2::new(-1.0, 0.0) } else { Vec2::new(1.0, 0.0) };
                return Some(Manifold { penetration: x_overlap, normal: normal });               
            } else {
                let normal = if n.y < 0.0 { Vec2::new(0.0, -1.0) } else { Vec2::new(0.0, 1.0) };
                return Some(Manifold { penetration: y_overlap, normal: normal });               
            }
        }
    }

    return None;
}

fn circle_circle_collision_manifold(a: &Circle, b: &Circle) -> Option<Manifold> {
    let n = b.position - a.position;
    let r = a.radius + b.radius;
    
    let mut distance = n.magnitude_squared();

    if distance > r * r {
        return None;
    }

    distance = distance.sqrt();

    //println!("n: {:?}, r: {}, d: {}", n, r, distance);

    if distance != 0.0 {
        Some(Manifold::new(r - distance, n / distance))
    } else {
        // Circles are in same position
        Some(Manifold::new(a.radius, Vec2::new(1.0, 0.0)))
    }
}

fn aabb_circle_collision_manifold(a: &AABB, b: &Circle) -> Option<Manifold> {
    let n = b.position - a.position();
    let a_extent = a.half_extent();

    // Clamp point to edges of the AABB
    let mut closest = Vec2::new(
        (-a_extent.x).max(a_extent.x.min(n.x)),
        (-a_extent.y).max(a_extent.y.min(n.y))
    );

    let mut inside = false;

    // Circle is inside the AABB, so we need to clamp the circle's center
    // to the closest edge
    if closest == n {
        inside = true;

        // Find closest axis
        if n.x.abs() > n.y.abs() {
            // Clamp to closest extent
            if closest.x > 0.0 {
                closest.x = a_extent.x
            } else {
                closest.x = -a_extent.x
            }
        } else {
            // Clamp to closest extent
            if closest.y > 0.0 {
                closest.y = a_extent.y
            } else {
                closest.y = -a_extent.y
            }
        }
    }

    let normal = n - closest;
    let mut distance = normal.magnitude_squared();
    
    // Early out of the radius is shorter than distance to closest point and
    // circle not inside the AABB
    if distance > b.radius * b.radius && !inside {
        return None;
    }

    distance = distance.sqrt();

    //println!("Normal: {:?}, n: {:?}, distance: {}", normal, n, distance);

    // Collision normal needs to be flipped to point outside if circle was
    // inside the AABB
    if inside {
        Some(Manifold::new(b.radius - distance, (normal * -1.0) / distance)) 
    } else {
        Some(Manifold::new(b.radius - distance, normal / distance)) 
    }
}

pub fn resolve_collision(a: &mut Entity, b: &mut Entity, manifold: Manifold) {
    resolve_bounce(&mut a.physics, &mut b.physics, manifold);
    fixup_position(&mut a.physics, &mut b.physics, manifold);
}

fn resolve_bounce(a: &mut Physics, b: &mut Physics, manifold: Manifold) {
    let relative_velocity = b.velocity - a.velocity;
    let velocity_along_normal = relative_velocity.dot_product(manifold.normal);

    //println!("Velocity along normal: {}", velocity_along_normal);

    if velocity_along_normal > 0.0 {
        // Do not resolve if velocities are separating
        return
    }

    let e = a.restitution.min(b.restitution);
    let j = (-(1.0 + e) * velocity_along_normal) / (a.inv_mass + b.inv_mass);
    
    let impulse = j * manifold.normal;

    //println!("Impulse: {:?}", impulse);
    //println!("Velocity before delta: {:?}, {:?}", a.velocity, b.velocity);

    a.velocity -= a.inv_mass * impulse;
    b.velocity += b.inv_mass * impulse;

    //println!("Velocity after delta: {:?}, {:?}", a.velocity, b.velocity);
}

fn fixup_position(a: &mut Physics, b: &mut Physics, manifold: Manifold) {

    let correction = manifold.penetration / (a.inv_mass + b.inv_mass) * manifold.normal;

    a.position -= a.inv_mass * correction;
    b.position += b.inv_mass * correction;
}

pub fn nearest_ray_intersection(ray: &Ray, entities: &[Entity]) -> Option<(usize, Vec2)> {
    let mut intersection = None;
    let mut min_distance = f32::INFINITY;

    for (i, entity) in entities.iter().enumerate() {
        let maybe_point = match entity.physics.collision_shape() {
            CollisionShape::AABB(aabb) => ray.box_intersection(&aabb),
            CollisionShape::Circle(circle) => ray.circle_intersection(&circle)
        };

        match maybe_point {
            Some(point) => {
                //println!("Intersection point: {:?}", point);
                let distance = ray.origin.distance(point);
                if distance < min_distance {
                    intersection = Some((i, point));
                    min_distance = distance;
                }
            }
            None => {}
        };
    }
    
    //println!("Intersection: {:?}", intersection);
    return intersection;
}

pub fn collision_point(entity: &Entity, entities: &[Entity]) -> Option<(usize, Vec2)> {
    let movement_line = LineSegment::new(entity.physics.position, entity.physics.position + entity.physics.velocity);
    
    nearest_ray_intersection(&Ray::from_segment(&movement_line), entities).and_then(|result| {
        if movement_line.has_point(result.1) {
            Some(result)
        } else {
            None
        }
    })

}

