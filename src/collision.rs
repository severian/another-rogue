use vec2::Vec2;
use entity::Entity;


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

    fixup_position(a, b, manifold);
}

pub fn fixup_position(a: &mut Entity, b: &mut Entity, manifold: Manifold) {

    let correction = manifold.penetration / (a.inv_mass + b.inv_mass) * manifold.normal;

    a.position -= a.inv_mass * correction;
    b.position -= b.inv_mass * correction;
}

