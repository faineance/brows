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
                       screen_width: u32,
                       text: &str)
                       -> Vec<PositionedGlyph<'a>> {
    use unicode_normalization::UnicodeNormalization;
    let mut result = Vec::new();
    let metrics = font.v_metrics(scale);
    let advance_height = metrics.ascent - metrics.descent + metrics.line_gap;
    let mut caret = point(0.0, metrics.ascent);// - advance_height / 2.);
    let mut last_glyph = None;
    for c in text.nfc() {
        if c.is_control() {
            match c {
                '\n' | '\r' => caret = point(0.0, caret.y + advance_height),
                _ => {}
            }
            continue;
        }
        let cur = if let Some(g) = font.glyph(c) {
            g
        } else {
            continue;
        };
        if let Some(id) = last_glyph.take() {
            caret.x += font.pair_kerning(scale, id, cur.id());
        }
        last_glyph = Some(cur.id());
        let glyph = cur.scaled(scale).positioned(caret);
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }

    result
}
