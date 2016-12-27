use std;
use sdl2::render::Renderer;
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
    fn draw_player(&mut self, player: &Player, physics: &Physics, now: u32);
    fn draw_enemy(&mut self, enemy: &Enemy, physics: &Physics);
    fn draw_entity(&mut self, entity: &Entity, now: u32);
}

impl<'a> EntityRenderer for Renderer<'a> {

    fn draw_shape(&mut self, physics: &Physics, color: Color) {
        match physics.shape {
            Shape::Rect { extent } => {
                self.set_draw_color(color);
                self.fill_rect(Rect::from_center(physics.position, extent.x as u32, extent.y as u32))
            }
            Shape::Circle { radius } =>
                self.filled_circle(physics.position.x as i16, physics.position.y as i16, radius as i16, color)
        }.expect("Draw didn't work")
    }

    fn draw_player(&mut self, player: &Player, physics: &Physics, now: u32) {
        self.set_draw_color(Color::RGB(0, 0, 255));
        self.draw_line(physics.position.into(), player.looking_at.into()).expect("Draw didn't work");

        self.draw_shape(physics, Color::RGB(0, 255, 0));

        player.gun_state(now).map(|gun_state| {
            let los_ray = Ray::from_segment(&LineSegment::new(physics.position, player.looking_at));
            los_ray.shape_intersection(&physics.collision_shape()).map(|player_gun_intersection| {
                let (radius, color) = match gun_state {
                    GunState::PewPew { charge } => (charge * 4.0, Color::RGB(255, 255, 0)),
                    GunState::Boom { charge } => ((charge * 2.0) + 4.0, Color::RGB(255, 0, 0))
                };

                self.filled_circle(player_gun_intersection.x as i16, player_gun_intersection.y as i16, radius as i16, color).expect("Draw didn't work")
            })
        });
    }

    fn draw_enemy(&mut self, enemy: &Enemy, physics: &Physics) {
        self.filled_circle(physics.position.x as i16, physics.position.y as i16, enemy.inner_radius as i16, Color::RGB(255, 0, 0));


        if enemy.has_shield() {
            match physics.shape {
                Shape::Circle { radius } => {
                    let num_ticks = 12;
                    let angle_step = (std::f32::consts::PI * 2.0) / num_ticks as f32;

                    for tick in 0..num_ticks {
                        let angle = tick as f32 * angle_step;
                        let x = physics.position.x + radius * angle.cos();
                        let y = physics.position.y + radius * angle.sin();

                        self.filled_circle(x as i16, y as i16, 3, Color::RGB(75, 162, 153));
                    }

                }
                _ => {}
            }
        }
    }

    fn draw_entity(&mut self, entity: &Entity, now: u32) {
        match entity.entity_type {
            EntityType::Player(ref player) => self.draw_player(player, &entity.physics, now),
            EntityType::Enemy(ref enemy) => self.draw_enemy(enemy, &entity.physics),
            _ => {
               let color = match entity.entity_type {
                   EntityType::Wall => Color::RGB(0, 0, 0),
                   EntityType::Bullet(bullet) => bullet.color(),
                   EntityType::Animation(animation) => animation.color,
                   _ => panic!("wrong entity")
               };

               self.draw_shape(&entity.physics, color);
            }
        };
    }

}


