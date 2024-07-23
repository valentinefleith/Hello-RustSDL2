use crate::context::Context;
use crate::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::VideoSubsystem;
use std::time::Duration;

pub struct GraphicWindow {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    renderer: Renderer,
    context: Context,
}

impl GraphicWindow {
    pub fn new(window_title: &str, height: u32, width: u32) -> Result<GraphicWindow, String> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(window_title, height, width)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut renderer = Renderer::new(window)?;
        let mut context = Context::new();
        Ok(GraphicWindow {
            sdl_context,
            video_subsystem,
            renderer,
            context,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        'running: loop {
            self.renderer.draw(&self.context)?;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(1, 1_000_000_000u32 / 30));
        }
        Ok(())
    }
}
