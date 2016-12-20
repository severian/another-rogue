use std::f32;

use vec2::Vec2;
use entity::Entity;
use line::LineSegment;


#[derive(Debug, Copy, Clone)]
pub struct Manifold {
    pub penetration: f32,
    pub normal: Vec2
}

// from https://gamedevelopment.tutsplus.com/tutorials/how-to-create-a-custom-2d-physics-engine-the-basics-and-impulse-resolution--gamedev-6331

pub fn collision_manifold(a: Entity, b: Entity) -> Option<Manifold> {
    let n = b.position - a.position;

    let abox = a.aabb();
    let bbox = b.aabb();

    //println!("Normal: {:?}", n);
    //println!("A box: {:?}", abox);
    //println!("B box: {:?}", bbox);

    let x_overlap = abox.extent_x() + bbox.extent_x() - n.x.abs();

    if x_overlap > 0.0 {
        let y_overlap = abox.extent_y() + bbox.extent_y() - n.y.abs();

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

pub fn resolve_collision(a: &mut Entity, b: &mut Entity, manifold: Manifold) {
    resolve_bounce(a, b, manifold);
    fixup_position(a, b, manifold);
}

fn resolve_bounce(a: &mut Entity, b: &mut Entity, manifold: Manifold) {
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

    //println!("Velocity before delta: {:?}", a.velocity);

    a.velocity -= a.inv_mass * impulse;
    b.velocity += b.inv_mass * impulse;

    //println!("Velocity after delta: {:?}", a.velocity);

}

fn fixup_position(a: &mut Entity, b: &mut Entity, manifold: Manifold) {

    let correction = manifold.penetration / (a.inv_mass + b.inv_mass) * manifold.normal;

    a.position -= a.inv_mass * correction;
    b.position -= b.inv_mass * correction;
}


pub fn nearest_line_intersection(line: LineSegment, entities: &[Entity]) -> Option<(Entity, Vec2)> {
    let mut intersection = None;
    let mut min_distance = f32::INFINITY;

    for entity in entities {
        for side in entity.aabb().line_segments().iter() {
            match line.intersection(*side) {
                Some(point) => {
                    //println!("Intersection point: {:?}", point);
                    let distance = line.start.distance(point);
                    if distance < min_distance {
                        intersection = Some((*entity, point));
                        min_distance = distance;
                    }
                }
                None => {}
            }
        }
    }
    
    //println!("Intersection: {:?}", intersection);
    return intersection;
}

