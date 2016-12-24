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
use player::Player;
use collision::nearest_ray_intersection;


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
    fn draw_player(&mut self, player: &Player, entity: &Entity);
    fn draw_entity(&mut self, entity: &Entity);
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

    fn draw_player(&mut self, player: &Player, entity: &Entity) {
        self.set_draw_color(Color::RGB(0, 0, 255));
        self.draw_line(entity.physics.position.into(), player.looking_at.into()).expect("Draw didn't work");

        self.draw_shape(&entity.physics, Color::RGB(0, 255, 0));

        match nearest_ray_intersection(&Ray::from_segment(&LineSegment::new(entity.physics.position, player.looking_at)), &[*entity]) {
            Some((_, player_gun_intersection)) =>
                self.filled_circle(player_gun_intersection.x as i16, player_gun_intersection.y as i16, 4, Color::RGB(255, 255, 0)).expect("Draw didn't work"),
            None => {}
        }
    }

    fn draw_entity(&mut self, entity: &Entity) {
        match entity.entity_type {
            EntityType::Player(ref player) => self.draw_player(player, entity),
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


