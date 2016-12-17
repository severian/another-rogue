extern crate sdl2;

mod vec2;
mod entity;
mod collision;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use entity::{Level, make_wall};
use collision::{Manifold, collision_manifold, resolve_collision};
use vec2::Vec2;

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
    level.walls.push(make_wall(20.0, 20.0, Vec2::new(200.0, 200.0)));
    level.walls.push(make_wall(20.0, 20.0, Vec2::new(400.0, 400.0)));

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

        level.player.position += level.player.velocity;

        for wall in &mut level.walls {
            match collision_manifold(level.player, *wall) {
                Some(Manifold { normal, .. }) => {
                    //println!("Collision normal: {:?}", normal);
                    resolve_collision(&mut level.player, wall, normal);
                }
                None => {}
            }
        }
        level.player.velocity += level.player.acceleration - level.player.velocity * DRAG;

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(255, 255, 255));
        for wall in &level.walls {
            renderer.fill_rect(Rect::new((wall.position.x - wall.aabb().extent_x()) as i32, (wall.position.y - wall.aabb().extent_y()) as i32, wall.width as u32, wall.height as u32)).expect("Draw didn't work");
        }

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.fill_rect(Rect::new((level.player.position.x - level.player.aabb().extent_x()) as i32, (level.player.position.y - level.player.aabb().extent_y()) as i32, level.player.width as u32, level.player.height as u32)).expect("Draw didn't work");

        renderer.present();
 
        // println!("Ticks: {}", timer.ticks());
    }
}

