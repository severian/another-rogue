extern crate sdl2;

mod vec2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use vec2::Vec2;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 20.0;

const ACCELERATION: f32 = 1.0;
const DRAG: f32 = 0.1;


const ORIGIN_VEC: Vec2 = Vec2 { x: 0.0, y: 0.0 };

struct Player {
    width: f32,
    height: f32,
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2
}

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

    let mut player = Player { 
        width: PLAYER_WIDTH,
        height: PLAYER_HEIGHT,
        position: Vec2 { x: (WINDOW_WIDTH / 2.0) - (PLAYER_WIDTH / 2.0), y: (WINDOW_HEIGHT / 2.0) - (PLAYER_HEIGHT / 2.0) },
        velocity: ORIGIN_VEC,
        acceleration: ORIGIN_VEC
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                    println!("KEYDOWN, repeat: {}", repeat);
                    if !repeat {
                        match keycode {
                            Keycode::Right => {
                                player.acceleration.x += ACCELERATION;
                            },
                            Keycode::Left => {
                                player.acceleration.x -= ACCELERATION;
                            },
                            Keycode::Down => {
                                player.acceleration.y += ACCELERATION;
                            },
                            Keycode::Up => {
                                player.acceleration.y -= ACCELERATION;
                            },
                            _ => {}
                        }
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    println!("KEYUP");
                    match keycode {
                        Keycode::Right => {
                            player.acceleration.x -= ACCELERATION;
                        },
                        Keycode::Left => {
                            player.acceleration.x += ACCELERATION;
                        },
                        Keycode::Down => {
                            player.acceleration.y -= ACCELERATION;
                        },
                        Keycode::Up => {
                            player.acceleration.y += ACCELERATION;
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        player.position += player.velocity;

        //if player.position.x > WINDOW_WIDTH as f32 {
        //    player.position.x = 0.0;
        //} else if player.position.x < 0.0 {
        //    player.position.x = WINDOW_WIDTH as f32;
        //}
        //if player.position.y > WINDOW_HEIGHT as f32 {
        //    player.position.y = 0.0;
        //} else if player.position.y < 0.0 {
        //    player.position.y = WINDOW_HEIGHT as f32;
        //}
 
        let mut collided = false;
        if player.position.x + player.width > WINDOW_WIDTH {
            player.position.x = WINDOW_WIDTH - player.width;
            collided = true;
        } else if player.position.x < 0.0 {
            player.position.x = 0.0;
            collided = true;
        }
        if player.position.y + player.height > WINDOW_HEIGHT {
            player.position.y = WINDOW_HEIGHT - player.height;
            collided = true;
        } else if player.position.y < 0.0 {
            player.position.y = 0.0;
            collided = true;
        }

        if collided {
            player.acceleration *= -1.0;
        }

        player.velocity += player.acceleration - player.velocity * DRAG;

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(Color::RGB(255, 0, 0));
        renderer.fill_rect(Rect::new(player.position.x as i32, player.position.y as i32, player.width as u32, player.height as u32)).expect("Draw didn't work");

        renderer.present();
 
        // println!("Ticks: {}", timer.ticks());
        // The rest of the game loop goes here...
    }
}

