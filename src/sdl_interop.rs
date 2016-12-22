use sdl2::render::Renderer;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::mouse::MouseState;

use vec2::Vec2;
use entity::Entity;
use shape::Shape;


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
    fn draw_entity(&mut self, entity: &Entity, color: Color);
}

impl<'a> EntityRenderer for Renderer<'a> {

    fn draw_entity(&mut self, entity: &Entity, color: Color) {
        match entity.physics.shape {
            Shape::Rect { extent } => {
                self.set_draw_color(color);
                self.fill_rect(Rect::from_center(entity.physics.position, extent.x as u32, extent.y as u32))
            }
            Shape::Circle { radius } =>
                self.filled_circle(entity.physics.position.x as i16, entity.physics.position.y as i16, radius as i16, color)
        }.expect("Draw didn't work")
    }

}


