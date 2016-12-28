extern crate sdl2;

mod render;
mod vec2;
mod entity;
mod collision;
mod line;
mod shape;
mod ray;
mod animation;
mod player;
mod enemy;
mod bullet;

use std::time::{Duration, Instant};

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::gfx::framerate::FPSManager;

use render::EntityRenderer;
use entity::{Level, make_wall, make_circle_wall, make_bullet, make_animation, make_enemy};
use collision::{collision_manifold, resolve_collision, nearest_ray_intersection, collision_point};
use vec2::Vec2;
use line::LineSegment;
use ray::Ray;
use entity::EntityType;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const ACCELERATION: f32 = 1.0;
const DRAG: f32 = 0.1;

const FPS: u32 = 60;


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

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(FPS).expect("Setting framerate didn't work");

    let mut level = Level::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    level.collision_entities.push(make_wall(40.0, 40.0, Vec2::new(200.0, 200.0)));
    level.collision_entities.push(make_wall(40.0, 40.0, Vec2::new(400.0, 400.0)));
    level.collision_entities.push(make_circle_wall(20.0, Vec2::new(500.0, 400.0)));

    level.collision_entities.push(make_enemy(Vec2::new(600.0, 200.0)));

    'running: loop {
        let delta = fps_manager.delay();
        //println!("Frame time delta: {}", delta);

        for entity in &mut level.collision_entities {
            entity.update(delta)
        }

        let ticks = timer.ticks();

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
                                level.player_mut().physics.acceleration.x += ACCELERATION;
                            },
                            Keycode::Left => {
                                level.player_mut().physics.acceleration.x -= ACCELERATION;
                            },
                            Keycode::Down => {
                                level.player_mut().physics.acceleration.y += ACCELERATION;
                            },
                            Keycode::Up => {
                                level.player_mut().physics.acceleration.y -= ACCELERATION;
                            },
                            _ => {}
                        }
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    //println!("KEYUP");
                    match keycode {
                        Keycode::Right => {
                            level.player_mut().physics.acceleration.x -= ACCELERATION;
                        },
                        Keycode::Left => {
                            level.player_mut().physics.acceleration.x += ACCELERATION;
                        },
                        Keycode::Down => {
                            level.player_mut().physics.acceleration.y -= ACCELERATION;
                        },
                        Keycode::Up => {
                            level.player_mut().physics.acceleration.y += ACCELERATION;
                        },
                        _ => {}
                    }
                }
                Event::MouseButtonDown {..} => {
                    level.player_mut().player_mut().start_gun_charging();
                }
                Event::MouseButtonUp { x, y, .. } => {
                    level.player_mut().player_mut().fire_gun().map(|bullet_type| {
                        let bullet = make_bullet(level.player(), bullet_type, Vec2::from_ints(x, y));
                        level.bullets.push(bullet)
                    });
                }
                _ => {}
            }
        }


        for entity in &mut level.collision_entities {
            entity.physics.position += entity.physics.velocity;
        }

        for bullet in &mut level.bullets {
            bullet.physics.position += bullet.physics.velocity;
        }

        for i in 0..level.collision_entities.len() {
            let (a, b) = level.collision_entities.split_at_mut(i + 1);
            let entity_a = a.last_mut().unwrap();
            for entity_b in b {
                match collision_manifold(entity_a, entity_b) {
                    Some(manifold) => {
                        //println!("Collision manifold: {:?}", manifold);
                        resolve_collision(entity_a, entity_b, manifold);
                    }
                    None => {}
                }
            }
        }

        for entity in &mut level.collision_entities {
            entity.physics.velocity += entity.physics.acceleration - entity.physics.velocity * DRAG;
        }

        //println!("Player velocity: {:?}", level.player.velocity);

        {
            let animations = &mut level.animations;
            let collision_entities = &mut level.collision_entities;
            level.bullets.retain(|bullet| {
                match collision_point(bullet, collision_entities) {
                    Some((index, point)) => {
                        animations.push(make_animation(ticks, bullet.bullet().color(), point));

                        match collision_entities.get_mut(index).unwrap().entity_type {
                            EntityType::Enemy(ref mut enemy) => {
                                enemy.take_hit(bullet)
                            }
                            _ => {}
                        }

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
        let los_ray = Ray::from_segment(&LineSegment::new(level.player().physics.position, mouse_state.into()));
        
        let los_end = match nearest_ray_intersection(&los_ray, &level.non_player_collision_entities()) {
            Some((_, p)) => p,
            None => los_ray.origin + (800.0 * los_ray.direction).normalize()
        };

        level.player_mut().player_mut().looking_at = los_end;

        renderer.set_draw_color(Color::RGB(88, 110, 117));
        renderer.clear();

        for wall in &level.collision_entities {
            renderer.draw_entity(wall);
        }

        for bullet in &level.bullets {
            renderer.draw_entity(bullet);
        }

        for entity in &level.animations {
            let color = entity.animation().color;
            let size = 1 * entity.animation().step(ticks);
            renderer.filled_circle(entity.physics.position.x as i16, entity.physics.position.y as i16, (size / 2) as i16, color).expect("Draw didn't work");
        }

        renderer.present();

 
        // println!("Ticks: {}", timer.ticks());
    }
}

