
use std::borrow::Cow;
use glium;
use rusttype::gpu_cache::Cache;
use glium::backend::glutin_backend::GlutinFacade;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::Rect;
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
    cache_tex: glium::texture::Texture2d,
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
            cache_tex: glium::texture::Texture2d::with_format(
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
}