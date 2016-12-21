extern crate sdl2;

mod vec2;
mod entity;
mod collision;
mod line;
mod aabb;
mod ray;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;

use entity::{Level, make_wall};
use collision::{collision_manifold, resolve_collision, nearest_ray_intersection};
use vec2::Vec2;
use line::LineSegment;
use ray::Ray;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const ACCELERATION: f32 = 1.0;
const DRAG: f32 = 0.1;



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800 as u32, 600 as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    // let mut timer = sdl_context.timer().unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut level = Level::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    level.walls.push(make_wall(40.0, 40.0, Vec2::new(200.0, 200.0)));
    level.walls.push(make_wall(40.0, 40.0, Vec2::new(400.0, 400.0)));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    //println!("KEYDOWN, repeat: {}", repeat);
                    if !repeat {
                        match keycode {
                            Keycode::Right => {
                                level.player.acceleration.x += ACCELERATION;
                            },
                            Keycode::Left => {
                                level.player.acceleration.x -= ACCELERATION;
                            },
                            Keycode::Down => {
                                level.player.acceleration.y += ACCELERATION;
                            },
                            Keycode::Up => {
                                level.player.acceleration.y -= ACCELERATION;
                            },
                            _ => {}
                        }
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    //println!("KEYUP");
                    match keycode {
                        Keycode::Right => {
                            level.player.acceleration.x -= ACCELERATION;
                        },
                        Keycode::Left => {
                            level.player.acceleration.x += ACCELERATION;
                        },
                        Keycode::Down => {
                            level.player.acceleration.y -= ACCELERATION;
                        },
                        Keycode::Up => {
                            level.player.acceleration.y += ACCELERATION;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        let mouse_state = event_pump.mouse_state();

        level.player.position += level.player.velocity;

        for wall in &mut level.walls {
            match collision_manifold(level.player, *wall) {
                Some(manifold) => {
                    //println!("Collision normal: {:?}", normal);
                    resolve_collision(&mut level.player, wall, manifold);
                }
                None => {}
            }
        }
        level.player.velocity += level.player.acceleration - level.player.velocity * DRAG;

        //println!("Player velocity: {:?}", level.player.velocity);


        let mouse_pos = Vec2::new(mouse_state.x() as f32, mouse_state.y() as f32);
        let gun_ray = Ray::from_segment(LineSegment::new(level.player.position, mouse_pos));
        
        let gun_los_end = match nearest_ray_intersection(gun_ray, &level.walls) {
            Some((_, p)) => p,
            None => gun_ray.origin + (800.0 * gun_ray.direction)
        };
        //let debug_walls = vec![*level.walls.last().unwrap()];
        //let gun_los_end = match nearest_ray_intersection(gun_ray, &debug_walls) {
        //    Some((_, p)) => {
        //        println!("--------");
        //        p
        //    },
        //    None => gun_ray.origin + (800.0 * gun_ray.direction)
        //};

        renderer.set_draw_color(Color::RGB(88, 110, 117));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        for wall in &level.walls {
            renderer.fill_rect(Rect::from_center(wall.position, wall.width as u32, wall.height as u32)).expect("Draw didn't work");
        }

        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.draw_line(level.player.position.into(), gun_los_end.into()).expect("Draw didn't work");

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.fill_rect(Rect::from_center(level.player.position, level.player.width as u32, level.player.height as u32)).expect("Draw didn't work");

        renderer.present();

 
        // println!("Ticks: {}", timer.ticks());
    }
}

impl Into<Point> for Vec2 {
    fn into(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

