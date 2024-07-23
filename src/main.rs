extern crate sdl2;

use hello_rustsdl2::graphic_window::GraphicWindow;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    let mut graphic_window = GraphicWindow::new("hello-rustsdl2", 800, 600)?;
    graphic_window.run()?;
    Ok(())
}
