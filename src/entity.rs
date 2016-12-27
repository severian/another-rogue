use std::iter::Chain;
use std::slice::IterMut;

use sdl2::pixels::Color;

use vec2;
use vec2::Vec2;
use shape::{Shape, CollisionShape, AABB, Circle};
use ray::Ray;
use line::LineSegment;
use animation::Animation;
use player::Player;
use enemy::Enemy;
use bullet::{Bullet, BulletType};

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 20.0;

const WALL_THICKNESS: f32 = 20.0;

#[derive(Debug, Copy, Clone)]
pub enum EntityType {
    Player(Player),
    Wall,
    Bullet(Bullet),
    Animation(Animation),
    Enemy(Enemy)
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

    pub fn player(&self) -> &Player {
        match self.entity_type {
            EntityType::Player(ref player) => player,
            _ => panic!("Tried to get a player from {:?}", self)
        }
    }

    pub fn player_mut(&mut self) -> &mut Player {
        match self.entity_type {
            EntityType::Player(ref mut player) => player,
            _ => panic!("Tried to get a player from {:?}", self)
        }
    }

    pub fn bullet(&self) -> &Bullet {
        match self.entity_type {
            EntityType::Bullet(ref bullet) => bullet,
            _ => panic!("Tried to get a bullet from {:?}", self)
        }
    }

    pub fn animation(&self) -> &Animation {
        match self.entity_type {
            EntityType::Animation(ref animation) => animation,
            _ => panic!("Tried to get an animation from {:?}", self)
        }
    }

    pub fn update_physics(&mut self) {
        match self.entity_type {
            EntityType::Enemy(ref enemy) => enemy.update_physics(&mut self.physics),
            _ => {}
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

    pub collision_entities: Vec<Entity>,

    pub bullets: Vec<Entity>,
    pub animations: Vec<Entity>
}


impl Level {
    pub fn new(width: f32, height: f32) -> Level {
        Level {
            width: width,
            height: height,

            collision_entities: vec![
                make_player(width, height),

                make_wall(width, WALL_THICKNESS, Vec2::new(width / 2.0, WALL_THICKNESS / 2.0)),
                make_wall(WALL_THICKNESS, height - (2.0 * WALL_THICKNESS), Vec2::new(width - (WALL_THICKNESS / 2.0), height / 2.0)),
                make_wall(width, WALL_THICKNESS, Vec2::new(width / 2.0, height - (WALL_THICKNESS / 2.0))),
                make_wall(WALL_THICKNESS, height - (2.0 * WALL_THICKNESS), Vec2::new(WALL_THICKNESS / 2.0, height / 2.0))
           ],

           bullets: vec![],
           animations: vec![]
        }
    }

    pub fn player(&self) -> &Entity {
        self.collision_entities.first().unwrap()
    }

    pub fn player_mut(&mut self) -> &mut Entity {
        self.collision_entities.first_mut().unwrap()
    }

    pub fn non_player_collision_entities(&self) -> &[Entity] {
        self.collision_entities.split_first().unwrap().1
    }

}

pub fn make_player(level_width: f32, level_height: f32) -> Entity {
    Entity::new(
        EntityType::Player(Player::new()),
        Physics {
            //shape: Shape::Rect { extent: Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT) },
            shape: Shape::Circle { radius: PLAYER_WIDTH / 2.0 },
            position: Vec2::new(level_width / 2.0, level_height / 2.0),
            velocity: vec2::ORIGIN,
            acceleration: vec2::ORIGIN,

            restitution: 1.5,
            inv_mass: 1.0 / 20.0
        }
    )
}
pub fn make_enemy(position: Vec2) -> Entity {
    Entity::new(
        EntityType::Enemy(Enemy::new(10.0)),
        Physics {
            //shape: Shape::Rect { extent: Vec2::new(30.0, 30.0) },
            shape: Shape::Circle { radius: 15.0 },
            position: position,
            velocity: vec2::ORIGIN,
            acceleration: vec2::ORIGIN,

            restitution: 1.5,
            inv_mass: 1.0 / 50.0
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

            restitution: 1.5,
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

pub fn make_bullet(player: &Entity, bullet_type: BulletType, fired_at: Vec2) -> Entity {
    let bullet_ray = Ray::from_segment(&LineSegment::new(player.physics.position, fired_at));
    let (radius, velocity) = match bullet_type {
        BulletType::PewPew => (2.0, 20.0),
        BulletType::Boom => (4.0, 12.0)
    };

    Entity::new(
        EntityType::Bullet(Bullet::new(bullet_type)),
        Physics {
            shape: Shape::Circle { radius: radius },
            position: player.physics.position,
            velocity: bullet_ray.direction.normalize() * velocity,
            acceleration: vec2::ORIGIN,

            restitution: 0.0,
            inv_mass: 0.0
        }
    )
}

pub fn make_animation(start_time: u32, color: Color, position: Vec2) -> Entity {
    Entity::new(
        EntityType::Animation(Animation::new(start_time, 16, 250, color)),
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

