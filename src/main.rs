extern crate sdl2;

mod sdl_interop;
mod vec2;
mod entity;
mod collision;
mod line;
mod shape;
mod ray;
mod animation;

use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

use sdl_interop::EntityRenderer;
use entity::{Level, make_wall, make_circle_wall, make_bullet, make_animation};
use collision::{collision_manifold, resolve_collision, nearest_ray_intersection, collision_point};
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

    let mut timer = sdl_context.timer().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut level = Level::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    level.walls.push(make_wall(40.0, 40.0, Vec2::new(200.0, 200.0)));
    level.walls.push(make_wall(40.0, 40.0, Vec2::new(400.0, 400.0)));
    //level.walls.push(make_circle_wall(20.0, Vec2::new(500.0, 400.0)));

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
                                level.player.physics.acceleration.x += ACCELERATION;
                            },
                            Keycode::Left => {
                                level.player.physics.acceleration.x -= ACCELERATION;
                            },
                            Keycode::Down => {
                                level.player.physics.acceleration.y += ACCELERATION;
                            },
                            Keycode::Up => {
                                level.player.physics.acceleration.y -= ACCELERATION;
                            },
                            _ => {}
                        }
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    //println!("KEYUP");
                    match keycode {
                        Keycode::Right => {
                            level.player.physics.acceleration.x -= ACCELERATION;
                        },
                        Keycode::Left => {
                            level.player.physics.acceleration.x += ACCELERATION;
                        },
                        Keycode::Down => {
                            level.player.physics.acceleration.y -= ACCELERATION;
                        },
                        Keycode::Up => {
                            level.player.physics.acceleration.y += ACCELERATION;
                        },
                        _ => {}
                    }
                }
                Event::MouseButtonUp { x, y, .. } => {
                    let bullet = make_bullet(level.player, Vec2::from_ints(x, y));
                    level.bullets.push(bullet)
                }
                _ => {}
            }
        }

        let ticks = timer.ticks();

        level.player.physics.position += level.player.physics.velocity;

        for wall in &mut level.walls {
            match collision_manifold(&level.player, wall) {
                Some(manifold) => {
                    println!("Collision manifold: {:?}", manifold);
                    resolve_collision(&mut level.player, wall, manifold);
                }
                None => {}
            }
        }
        level.player.physics.velocity += level.player.physics.acceleration - level.player.physics.velocity * DRAG;

        //println!("Player velocity: {:?}", level.player.velocity);


        for bullet in &mut level.bullets {
            bullet.physics.position += bullet.physics.velocity;
        }

        {
            let walls = &level.walls;
            let animations = &mut level.animations;
            level.bullets.retain(|bullet| {
                match collision_point(*bullet, walls) {
                    Some((_, point)) => {
                        animations.push(make_animation(ticks, point));
                        false
                    }
                    None => true
                }
            });
        }

        level.animations.retain(|entity| {
            !entity.animation().is_expired(ticks)
        });


        let mouse_state = event_pump.mouse_state();
        let gun_ray = Ray::from_segment(&LineSegment::new(level.player.physics.position, mouse_state.into()));
        
        let gun_los_end = match nearest_ray_intersection(&gun_ray, &level.walls) {
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
            renderer.draw_entity(wall, Color::RGB(0, 0, 0));
        }

        renderer.set_draw_color(Color::RGB(0, 0, 255));
        renderer.draw_line(level.player.physics.position.into(), gun_los_end.into()).expect("Draw didn't work");

        renderer.draw_entity(&level.player, Color::RGB(255, 0, 0));

        for bullet in &level.bullets {
            renderer.draw_entity(bullet, Color::RGB(255, 255, 0));
        }

        renderer.set_draw_color(Color::RGB(255, 255, 0));
        for entity in &level.animations {
            let size = 1 * entity.animation().step(ticks);
            renderer.filled_circle(entity.physics.position.x as i16, entity.physics.position.y as i16, (size / 2) as i16, Color::RGB(255, 255, 0)).expect("Draw didn't work");
            //renderer.fill_rect(Rect::from_center(entity.physics.position, size, size)).expect("Draw didn't work");
        }

        renderer.present();

 
        // println!("Ticks: {}", timer.ticks());
    }
}

