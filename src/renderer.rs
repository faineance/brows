
use std::borrow::Cow;

use glium::{self, Frame, VertexBuffer, Blend, Program, Surface, Display};
use rusttype::gpu_cache::Cache;
use glium::backend::glutin_backend::GlutinFacade;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::Rect;
use layout::layout_text;
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    colour: [f32; 4],
}
implement_vertex!(Vertex, position, tex_coords, colour);


static FONT_FRAGMENT_SHADER: &'static str = "version 140
uniform sampler2D tex;
in vec2 v_tex_coords;
in vec4 v_colour;
out vec4 f_colour;

void main() {
    f_colour = v_colour * vec4(1.0, 1.0, 1.0, texture(tex, v_tex_coords).r);
}
";

static FONT_VERTEX_SHADER: &'static str = "#version 140
in vec2 position;
in vec2 tex_coords;
in vec4 colour;

out vec2 v_tex_coords;
out vec4 v_colour;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_tex_coords = tex_coords;
    v_colour = colour;
}
";

pub struct Renderer<'a> {
    cache: Cache,
    font: Font<'a>,
    program: glium::Program,
    cache_texture: glium::texture::Texture2d,
    vertices: Vec<Vertex>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: &GlutinFacade, font_data: &'a [u8]) -> Renderer<'a> {

        let dpi_factor = display.get_window().unwrap().hidpi_factor();
        let (cache_width, cache_height) = (512 * dpi_factor as u32, 512 * dpi_factor as u32);

        Renderer {
            cache: Cache::new(cache_width, cache_height, 0.1, 0.1),
            font: FontCollection::from_bytes(font_data).into_font().unwrap(),
            program: program!(display, 140 => { vertex: FONT_VERTEX_SHADER, fragment: FONT_FRAGMENT_SHADER }).unwrap(),
            vertices: vec![],
            cache_texture: glium::texture::Texture2d::with_format(
                                display,
                                glium::texture::RawImage2d {
                                    data: Cow::Owned(vec![128u8; cache_width as usize * cache_height as usize]),
                                    width: cache_width,
                                    height: cache_height,
                                    format: glium::texture::ClientFormat::U8
                                },
                                glium::texture::UncompressedFloatFormat::U8,
                                glium::texture::MipmapsOption::NoMipmap)
                                .unwrap()
        }
    }
    pub fn draw_text(&mut self, display: &Display, target: &mut Frame, text: &str) {
        let (width, dpi_factor) = {
            let window = display.get_window().unwrap();
            (window.get_inner_size_pixels().unwrap().0, window.hidpi_factor())
        };
        let glyphs = layout_text(&self.font, Scale::uniform(24.0 * dpi_factor), width, &text);
        cache_glyphs(&mut self.cache, &mut self.cache_texture, &glyphs);
        let uniforms = uniform! {
            tex: self.cache_texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };
    }
}


fn cache_glyphs<'a>(cache: &mut Cache,
                    cache_texture: &mut glium::texture::Texture2d,
                    glyphs: &Vec<PositionedGlyph<'a>>) {
    glyphs.into_iter().map(|glyph| cache.queue_glyph(0, glyph.clone()));
    cache.cache_queued(|rect, data| {
            let r = glium::Rect {
                left: rect.min.x,
                bottom: rect.min.y,
                width: rect.width(),
                height: rect.height(),
            };
            let tex = glium::texture::RawImage2d {
                data: Cow::Borrowed(data),
                width: rect.width(),
                height: rect.height(),
                format: glium::texture::ClientFormat::U8,
            };
            cache_texture.main_level().write(r, tex);
        })
        .unwrap();
}