#[macro_use]
extern crate glium;
extern crate unicode_normalization;
extern crate rusttype;
extern crate arrayvec;
mod renderer;
mod layout;
mod cache;
use glium::{DisplayBuild, Surface};

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    let font_data = include_bytes!("arial.ttf");
    let mut renderer = renderer::Renderer::new(&display, font_data);
    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        renderer.draw_text(&mut target,"test");
        target.finish().unwrap();
        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
