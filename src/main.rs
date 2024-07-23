extern crate sdl2;

mod renderer;

use renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window;
use std::time::Duration;

const DOT_SIZE_IN_PXS: u32 = 20;

pub enum State {
    Running,
    Paused,
}

pub struct Point(pub i32, pub i32);

pub struct Context {
    pub food: Point,
    pub state: State,
}

impl Context {
    pub fn new() -> Context {
        Context {
            food: Point(3, 3),
            state: State::Paused,
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut context = Context::new();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        renderer.draw(&context)?;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("ESC PRESSED");
                    break 'running;
                }
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
    }
    Ok(())
}
