use vec2;
use vec2::Vec2;
use aabb::AABB;
use ray::Ray;
use line::LineSegment;

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 20.0;

const WALL_THICKNESS: f32 = 20.0;

#[derive(Debug, Copy, Clone)]
pub struct Entity {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    pub restitution: f32,
    pub inv_mass: f32
}

impl Entity {
    pub fn aabb(self) -> AABB {
        let extent = Vec2::new(self.width / 2.0, self.height / 2.0);
        AABB { 
            min: self.position - extent, 
            max: self.position + extent
        }
    }
}

pub struct Level {
    pub width: f32,
    pub height: f32,
    pub player: Entity,
    pub walls: Vec<Entity>,
    pub bullets: Vec<Entity>
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
           bullets: vec![]
        }
    }
}

pub fn make_player(level_width: f32, level_height: f32) -> Entity {
    Entity {
        width: PLAYER_WIDTH,
        height: PLAYER_HEIGHT,
        position: Vec2::new(level_width / 2.0, level_height / 2.0),
        velocity: vec2::ORIGIN,
        acceleration: vec2::ORIGIN,

        restitution: 1.0,
        inv_mass: 1.0 / 20.0
    }
}

pub fn make_wall(width: f32, height: f32, position: Vec2) -> Entity {
    Entity {
        width: width,
        height: height,
        position: position,
        velocity: vec2::ORIGIN,
        acceleration: vec2::ORIGIN,

        restitution: 50.0,
        inv_mass: 0.0
    }
}

pub fn make_bullet(player: Entity, fired_at: Vec2) -> Entity {
    let bullet_ray = Ray::from_segment(LineSegment::new(player.position, fired_at));

    Entity {
        width: 2.0,
        height: 2.0,
        position: player.position,
        velocity: bullet_ray.direction * 10.0,
        acceleration: vec2::ORIGIN,

        restitution: 0.0,
        inv_mass: 0.0
    }
}

