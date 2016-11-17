// use html5ever::parse_document;
// use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};
// use glium::{DisplayBuild, Surface};
// use glium::glutin;
// use html5ever::tendril::TendrilSink;
use rusttype::{FontCollection, Font, Scale, point, vector, PositionedGlyph};
use rusttype::gpu_cache::Cache;
use rusttype::Rect;

use std::borrow::Cow;


// pub fn parse(html: String) -> RcDom {
//     match parse_document()
// }

pub fn layout_text<'a>(font: &'a Font,
                       scale: Scale,
                       window_width: u32,
                       text: &str)
                       -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\n' | '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                _ => {}
            }
            continue;
        }
        let base_glyph = if let Some(glyph) = font.glyph(c) {
            glyph
        } else {
            continue;
        };
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);
        if let Some(bb) = glyph.pixel_bounding_box() {
            if bb.max.x > window_width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph = glyph.into_unpositioned().positioned(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }

    result
}
