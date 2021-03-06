use std::f32;

use sdl2::render::WindowCanvas;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::mouse::MouseState;

use vec2::Vec2;
use entity::{Entity, EntityType, Physics};
use ray::Ray;
use line::LineSegment;
use shape::Shape;
use player::{Player, GunState};
use enemy::Enemy;
use animation::Animation;


impl Into<Point> for Vec2 {
    fn into(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

impl Into<Vec2> for MouseState {
    fn into(self) -> Vec2 {
        Vec2::new(self.x() as f32, self.y() as f32)
    }
}

pub trait EntityRenderer {
    fn draw_shape(&mut self, physics: &Physics, color: Color);
    fn draw_player(&mut self, player: &Player, physics: &Physics);
    fn draw_enemy(&mut self, enemy: &Enemy, physics: &Physics);
    fn draw_animation(&mut self, animation: &Animation, physics: &Physics);
    fn draw_entity(&mut self, entity: &Entity);
}

impl EntityRenderer for WindowCanvas {

    fn draw_shape(&mut self, physics: &Physics, color: Color) {
        match physics.shape {
            Shape::Rect { extent } => {
                self.set_draw_color(color);
                self.fill_rect(Rect::from_center(physics.position, extent.x as u32, extent.y as u32))
            }
            Shape::Circle { radius } =>
                self.filled_circle(physics.position.x.round() as i16, physics.position.y.round() as i16, radius.round() as i16, color)
        }.expect("Draw didn't work")
    }

    fn draw_player(&mut self, player: &Player, physics: &Physics) {
        self.set_draw_color(Color::RGB(0, 0, 255));
        self.draw_line(physics.position, player.looking_at).expect("Draw didn't work");

        self.draw_shape(physics, Color::RGB(0, 255, 0));

        player.gun_state().map(|gun_state| {
            let los_ray = Ray::from_segment(&LineSegment::new(physics.position, player.looking_at));
            los_ray.shape_intersection(&physics.collision_shape()).map(|player_gun_intersection| {
                let (radius, color) = match gun_state {
                    GunState::PewPew { charge } => (charge * 4.0, Color::RGB(255, 255, 0)),
                    GunState::Boom { charge } => ((charge * 2.0) + 4.0, Color::RGB(255, 0, 0))
                };

                self.filled_circle(player_gun_intersection.x.round() as i16, player_gun_intersection.y.round() as i16, radius.round() as i16, color).expect("Draw didn't work")
            })
        });
    }

    fn draw_enemy(&mut self, enemy: &Enemy, physics: &Physics) {
        self.filled_circle(physics.position.x as i16, physics.position.y as i16, enemy.inner_radius as i16, Color::RGB(255, 0, 0)).expect("Draw didn't work");

        match physics.shape {
            Shape::Circle { radius } => {
                let draw_radius = radius - 3.0;
                let angle_step = (f32::consts::PI * 2.0) / enemy.shield_health.len() as f32;

                for (i, shield_health) in enemy.shield_health.iter().enumerate() {
                    if *shield_health > 0.0 {
                        let angle = i as f32 * angle_step;
                        let x = physics.position.x + draw_radius * angle.cos();
                        let y = physics.position.y + draw_radius * angle.sin();

                        self.filled_circle(x.round() as i16, y.round() as i16, 3, Color::RGB(75, 162, 153)).expect("Draw didn't work");
                    }
                }
            }
            _ => {}
        }
    }

    fn draw_animation(&mut self, animation: &Animation, physics: &Physics) {
        let size = 1 * animation.step();
        self.filled_circle(physics.position.x as i16, physics.position.y as i16, (size / 2) as i16, animation.color).expect("Draw didn't work");
    }

    fn draw_entity(&mut self, entity: &Entity) {
        match entity.entity_type {
            EntityType::Player(ref player) => self.draw_player(player, &entity.physics),
            EntityType::Enemy(ref enemy) => self.draw_enemy(enemy, &entity.physics),
            EntityType::Animation(ref animation) => self.draw_animation(animation, &entity.physics),
            _ => {
               let color = match entity.entity_type {
                   EntityType::Wall => Color::RGB(0, 0, 0),
                   EntityType::Bullet(bullet) => bullet.color(),
                   _ => panic!("wrong entity")
               };

               self.draw_shape(&entity.physics, color);
            }
        };
    }

}


