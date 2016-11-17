#[macro_use]
extern crate glium;
extern crate unicode_normalization;
extern crate rusttype;
extern crate arrayvec;
mod renderer;
mod layout;
mod cache;
use glium::{DisplayBuild, Surface};
use glium::glutin::{Event,VirtualKeyCode};

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let font_data = include_bytes!("arial.ttf");
    let mut renderer = renderer::Renderer::new(&display, font_data);
    let text = "Happy birthday to you
Happy birthday to you
Happy birthday
    ";
    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        renderer.draw_text(&mut target, text);
        target.finish().unwrap();
        for event in display.poll_events() {
            match event {
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) |
                Event::Closed => return,
                _ => (),
            }
        }
    }
}
