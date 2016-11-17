
use std::borrow::Cow;

use glium::{self, Frame, VertexBuffer, Blend, Program, Surface, Display};
use rusttype::gpu_cache::Cache;
use glium::backend::glutin_backend::GlutinFacade;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::Rect;

use arrayvec::ArrayVec;
use renderer::Vertex;
use layout::layout_text;
pub fn cache_glyphs<'a>(cache: &mut Cache,
                        cache_texture: &mut glium::texture::Texture2d,
                        glyphs: &Vec<PositionedGlyph<'a>>) {
    for glyph in glyphs {
        cache.queue_glyph(0, glyph.clone());
    }

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

pub fn retrieve_glyphs_from_cache<'a>(cache: &Cache,
                                      display: &Display,
                                      glyphs: &Vec<PositionedGlyph<'a>>)
                                      -> Vec<Vertex> {
    let colour = [0.0, 0.0, 0.0, 1.0];
    let (screen_width, screen_height) = {
        let (w, h) = display.get_framebuffer_dimensions();
        (w as f32, h as f32)
    };
    let origin = point(0.0, 0.0);
    glyphs.iter()
        .flat_map(|g| {
            if let Ok(Some((uv_rect, screen_rect))) = cache.rect_for(0, g) {
                let gl_rect = Rect {
                    min: origin +
                         (vector(screen_rect.min.x as f32 / screen_width - 0.5,
                                 1.0 - screen_rect.min.y as f32 / screen_height - 0.5)) *
                         2.0,
                    max: origin +
                         (vector(screen_rect.max.x as f32 / screen_width - 0.5,
                                 1.0 - screen_rect.max.y as f32 / screen_height - 0.5)) *
                         2.0,
                };
                ArrayVec::<[Vertex; 6]>::from([Vertex {
                                                   position: [gl_rect.min.x, gl_rect.max.y],
                                                   tex_coords: [uv_rect.min.x, uv_rect.max.y],
                                                   colour: colour,
                                               },
                                               Vertex {
                                                   position: [gl_rect.min.x, gl_rect.min.y],
                                                   tex_coords: [uv_rect.min.x, uv_rect.min.y],
                                                   colour: colour,
                                               },
                                               Vertex {
                                                   position: [gl_rect.max.x, gl_rect.min.y],
                                                   tex_coords: [uv_rect.max.x, uv_rect.min.y],
                                                   colour: colour,
                                               },
                                               Vertex {
                                                   position: [gl_rect.max.x, gl_rect.min.y],
                                                   tex_coords: [uv_rect.max.x, uv_rect.min.y],
                                                   colour: colour,
                                               },
                                               Vertex {
                                                   position: [gl_rect.max.x, gl_rect.max.y],
                                                   tex_coords: [uv_rect.max.x, uv_rect.max.y],
                                                   colour: colour,
                                               },
                                               Vertex {
                                                   position: [gl_rect.min.x, gl_rect.max.y],
                                                   tex_coords: [uv_rect.min.x, uv_rect.max.y],
                                                   colour: colour,
                                               }])
            } else {
                ArrayVec::new()
            }
        })
        .collect()

}
