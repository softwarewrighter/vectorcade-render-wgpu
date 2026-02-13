//! Tests for NullRenderer.

use glam::Vec2;
use vectorcade_render_wgpu::{NullRenderer, RenderStats, VectorRenderer};
use vectorcade_shared::Rgba;
use vectorcade_shared::draw::{DrawCmd, Line2, Stroke};
use vectorcade_shared::font::FontStyleId;

fn stroke() -> Stroke {
    Stroke::new(Rgba::WHITE, 1.0)
}

fn line(a: Vec2, b: Vec2) -> DrawCmd {
    DrawCmd::Line(Line2 {
        a,
        b,
        stroke: stroke(),
    })
}

#[test]
fn empty_commands() {
    let mut r = NullRenderer;
    assert_eq!(r.render(&[]), RenderStats::default());
}

#[test]
fn counts_lines() {
    let mut r = NullRenderer;
    let cmds = vec![line(Vec2::ZERO, Vec2::ONE), line(Vec2::ONE, Vec2::NEG_ONE)];
    let stats = r.render(&cmds);
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.polylines, 0);
}

#[test]
fn counts_polylines() {
    let mut r = NullRenderer;
    let cmds = vec![DrawCmd::Polyline {
        pts: vec![Vec2::ZERO, Vec2::X, Vec2::Y],
        closed: true,
        stroke: stroke(),
    }];
    assert_eq!(r.render(&cmds).polylines, 1);
}

#[test]
fn counts_text() {
    let mut r = NullRenderer;
    let cmds = vec![DrawCmd::Text {
        pos: Vec2::ZERO,
        text: "HI".into(),
        size_px: 12.0,
        color: Rgba::GREEN,
        style: FontStyleId::DEFAULT,
    }];
    assert_eq!(r.render(&cmds).text_runs, 1);
}

#[test]
fn ignores_non_drawing_commands() {
    let mut r = NullRenderer;
    let cmds = vec![
        DrawCmd::Clear { color: Rgba::BLACK },
        DrawCmd::PushTransform(glam::Mat3::IDENTITY),
        DrawCmd::PopTransform,
    ];
    assert_eq!(r.render(&cmds), RenderStats::default());
}
