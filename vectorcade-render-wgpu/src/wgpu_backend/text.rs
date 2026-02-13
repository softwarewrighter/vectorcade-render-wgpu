//! Text rendering via vector fonts.

use crate::tessellate::{Geometry, tessellate_polyline};
use glam::{Mat3, Vec2};
use vectorcade_fonts::FontRegistry;
use vectorcade_shared::draw::Stroke;
use vectorcade_shared::font::{FontStyleId, GlyphPathCmd};
use vectorcade_shared::Rgba;

/// Parameters for text rendering.
pub struct TextParams<'a> {
    pub registry: &'a FontRegistry,
    pub text: &'a str,
    pub pos: Vec2,
    pub size_px: f32,
    pub color: Rgba,
    pub style: FontStyleId,
}

/// Render text into geometry using the given font registry.
pub fn tessellate_text(params: &TextParams<'_>, transform: Option<&Mat3>, geom: &mut Geometry) {
    let Some(font) = params.registry.get(params.style) else { return };
    let stroke = Stroke::new(params.color, params.size_px * 0.08);
    let mut cursor_x = params.pos.x;

    for ch in params.text.chars() {
        if font.has_glyph(ch) {
            for path in font.glyph_paths(ch) {
                tessellate_glyph(&path.cmds, cursor_x, params.pos.y, params.size_px, &stroke, transform, geom);
            }
        }
        cursor_x += font.advance(ch) * params.size_px;
    }
}

fn tessellate_glyph(
    cmds: &[GlyphPathCmd],
    x_offset: f32,
    y_offset: f32,
    scale: f32,
    stroke: &Stroke,
    transform: Option<&Mat3>,
    geom: &mut Geometry,
) {
    let mut pts: Vec<[f32; 2]> = Vec::new();
    let mut closed = false;

    for cmd in cmds {
        match cmd {
            GlyphPathCmd::MoveTo(p) => {
                flush_path(&pts, closed, stroke, transform, geom);
                pts.clear();
                closed = false;
                pts.push(transform_glyph_pt(*p, x_offset, y_offset, scale));
            }
            GlyphPathCmd::LineTo(p) => {
                pts.push(transform_glyph_pt(*p, x_offset, y_offset, scale));
            }
            GlyphPathCmd::Close => {
                closed = true;
            }
        }
    }
    flush_path(&pts, closed, stroke, transform, geom);
}

fn transform_glyph_pt(p: Vec2, x_offset: f32, y_offset: f32, scale: f32) -> [f32; 2] {
    [x_offset + p.x * scale, y_offset + p.y * scale]
}

fn flush_path(
    pts: &[[f32; 2]],
    closed: bool,
    stroke: &Stroke,
    transform: Option<&Mat3>,
    geom: &mut Geometry,
) {
    if pts.len() >= 2 {
        tessellate_polyline(pts, closed, stroke, transform, geom);
    }
}
