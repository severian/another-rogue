use std::f32;

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

    pub fn animation_mut(&mut self) -> &mut Animation {
        match self.entity_type {
            EntityType::Animation(ref mut animation) => animation,
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

    pub fn non_player_collision_entities_mut(&mut self) -> &[Entity] {
        self.collision_entities.split_first_mut().unwrap().1
    }

    pub fn update(&mut self, time_delta: u32) {
        let (player, entities) = self.collision_entities.split_first_mut().unwrap();

        player.player_mut().update(time_delta);

        for entity in entities {
            match entity.entity_type {
                EntityType::Player(ref mut player) => player.update(time_delta),
                EntityType::Enemy(ref mut enemy) => enemy.update(&mut entity.physics, player, time_delta),
                _ => {}
            }
        }

        for animation in &mut self.animations {
            animation.animation_mut().update(time_delta)
        }
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
            shape: Shape::Circle { radius: 20.0 },
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
    let bullet_pos = bullet_ray.shape_intersection(&player.physics.collision_shape()).unwrap();
    let normal = bullet_ray.direction.normalize();

    let (radius, velocity) = match bullet_type {
        BulletType::PewPew => (2.0, 20.0),
        BulletType::Boom => (4.0, 12.0)
    };

    Entity::new(
        EntityType::Bullet(Bullet::new(bullet_type)),
        Physics {
            shape: Shape::Circle { radius: radius },
            position: bullet_pos + (normal * 0.0001),
            velocity: normal * velocity,
            acceleration: vec2::ORIGIN,

            restitution: 0.0,
            inv_mass: 0.0
        }
    )
}

pub fn make_animation(color: Color, position: Vec2) -> Entity {
    Entity::new(
        EntityType::Animation(Animation::new(16, 250, color)),
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

