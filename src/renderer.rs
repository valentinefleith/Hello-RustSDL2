use crate::context::{Context, Point, State};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub const DOT_SIZE_IN_PXS: u32 = 20;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;
        Ok(())
    }

    pub fn draw(&mut self, context: &Context) -> Result<(), String> {
        self.draw_background(context);
        self.draw_food(context)?;
        self.canvas.present();
        Ok(())
    }

    fn draw_background(&mut self, context: &Context) {
        let color = match context.state {
            State::Paused => Color::RGB(30, 30, 30),
            State::Running => Color::RGB(0, 0, 0),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_food(&mut self, context: &Context) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;
        Ok(())
    }
}
