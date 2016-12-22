use vec2;
use vec2::Vec2;
use shape::{Shape, CollisionShape, AABB, Circle};
use ray::Ray;
use line::LineSegment;
use animation::Animation;

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 20.0;

const WALL_THICKNESS: f32 = 20.0;

#[derive(Debug, Copy, Clone)]
pub enum EntityType {
    Player,
    Wall,
    Bullet,
    Animation(Animation)
}

#[derive(Debug, Copy, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub physics: Physics
}

impl Entity {
    pub fn new(entity_type: EntityType, physics: Physics) -> Entity {
        Entity { entity_type: entity_type, physics: physics }
    }

    pub fn animation(&self) -> Animation {
        match self.entity_type {
            EntityType::Animation(animation) => animation,
            _ => panic!("Tried to get an animation from {:?}", self)
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Physics {
    pub shape: Shape,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    pub restitution: f32,
    pub inv_mass: f32
}

impl Physics {

    pub fn collision_shape(&self) -> CollisionShape {
        match self.shape {
            Shape::Rect { extent } => {
                let half_extent = extent / 2.0;
                CollisionShape::AABB(AABB::new(self.position - half_extent, self.position + half_extent))
            }
            Shape::Circle { radius } => {
                CollisionShape::Circle(Circle::new(self.position, radius))
            }
        }
    }

}

pub struct Level {
    pub width: f32,
    pub height: f32,
    pub player: Entity,
    pub walls: Vec<Entity>,
    pub bullets: Vec<Entity>,
    pub animations: Vec<Entity>
}

impl Level {
    pub fn new(width: f32, height: f32) -> Level {
        Level {
            width: width,
            height: height,
            player: make_player(width, height),
            walls: vec![
                make_wall(width, WALL_THICKNESS, Vec2::new(width / 2.0, WALL_THICKNESS / 2.0)),
                make_wall(WALL_THICKNESS, height - (2.0 * WALL_THICKNESS), Vec2::new(width - (WALL_THICKNESS / 2.0), height / 2.0)),
                make_wall(width, WALL_THICKNESS, Vec2::new(width / 2.0, height - (WALL_THICKNESS / 2.0))),
                make_wall(WALL_THICKNESS, height - (2.0 * WALL_THICKNESS), Vec2::new(WALL_THICKNESS / 2.0, height / 2.0))
           ],
           bullets: vec![],
           animations: vec![]
        }
    }
}

pub fn make_player(level_width: f32, level_height: f32) -> Entity {
    Entity::new(
        EntityType::Player,
        Physics {
            //shape: Shape::Rect { extent: Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT) },
            shape: Shape::Circle { radius: PLAYER_WIDTH / 2.0 },
            position: Vec2::new(level_width / 2.0, level_height / 2.0),
            velocity: vec2::ORIGIN,
            acceleration: vec2::ORIGIN,

            restitution: 1.0,
            inv_mass: 1.0 / 20.0
        }
    )
}

pub fn make_wall(width: f32, height: f32, position: Vec2) -> Entity {
    Entity::new(
        EntityType::Wall,
        Physics {
            shape: Shape::Rect { extent: Vec2::new(width, height) },
            position: position,
            velocity: vec2::ORIGIN,
            acceleration: vec2::ORIGIN,

            restitution: 50.0,
            inv_mass: 0.0
        }
    )
}

pub fn make_circle_wall(radius: f32, position: Vec2) -> Entity {
    Entity::new(
        EntityType::Wall,
        Physics {
            shape: Shape::Circle { radius: radius },
            position: position,
            velocity: vec2::ORIGIN,
            acceleration: vec2::ORIGIN,

            restitution: 50.0,
            inv_mass: 0.0
        }
    )
}

pub fn make_bullet(player: Entity, fired_at: Vec2) -> Entity {
    let bullet_ray = Ray::from_segment(&LineSegment::new(player.physics.position, fired_at));

    Entity::new(
        EntityType::Bullet,
        Physics {
            shape: Shape::Circle { radius: 2.0 },
            position: player.physics.position,
            velocity: bullet_ray.direction * 20.0,
            acceleration: vec2::ORIGIN,

            restitution: 0.0,
            inv_mass: 0.0
        }
    )
}

pub fn make_animation(start_time: u32, position: Vec2) -> Entity {
    Entity::new(
        EntityType::Animation(Animation::new(start_time, 16, 250)),
        Physics {
            shape: Shape::Circle { radius: 0.0 },
            position: position,
            velocity: vec2::ORIGIN,
            acceleration: vec2::ORIGIN,

            restitution: 0.0,
            inv_mass: 0.0
        }
    )
}

