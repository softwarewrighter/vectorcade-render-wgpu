//! Integration tests for the full rendering pipeline.
//!
//! Tests all DrawCmd types with the NullRenderer to verify command processing.

use glam::{Mat3, Vec2};
use vectorcade_render_wgpu::{NullRenderer, VectorRenderer};
use vectorcade_shared::draw::{DrawCmd, Line2, Stroke};
use vectorcade_shared::font::FontStyleId;
use vectorcade_shared::Rgba;

/// Smoke test: verify wgpu device creation succeeds (headless, no surface).
#[cfg(feature = "wgpu-backend")]
#[test]
fn wgpu_device_creation_smoke_test() {
    // This test runs synchronously using pollster to block on async
    pollster::block_on(async {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None, // headless
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find GPU adapter");

        let (device, _queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("Failed to create device");

        // Verify we got a valid device by checking its limits
        let limits = device.limits();
        assert!(limits.max_texture_dimension_2d > 0, "Device should have valid limits");
    });
}

fn white_stroke(width: f32) -> Stroke {
    Stroke::new(Rgba::WHITE, width)
}

#[test]
fn renders_pong_like_scene() {
    let mut renderer = NullRenderer;
    let cmds = vec![
        DrawCmd::Clear { color: Rgba::BLACK },
        // Left paddle
        DrawCmd::Polyline {
            pts: vec![Vec2::new(-0.9, -0.2), Vec2::new(-0.9, 0.2), Vec2::new(-0.85, 0.2), Vec2::new(-0.85, -0.2)],
            closed: true,
            stroke: white_stroke(2.0),
        },
        // Right paddle
        DrawCmd::Polyline {
            pts: vec![Vec2::new(0.85, -0.15), Vec2::new(0.85, 0.15), Vec2::new(0.9, 0.15), Vec2::new(0.9, -0.15)],
            closed: true,
            stroke: white_stroke(2.0),
        },
        // Ball
        DrawCmd::Polyline {
            pts: vec![Vec2::new(-0.02, -0.02), Vec2::new(0.02, -0.02), Vec2::new(0.02, 0.02), Vec2::new(-0.02, 0.02)],
            closed: true,
            stroke: white_stroke(2.0),
        },
        // Center line
        DrawCmd::Line(Line2 { a: Vec2::new(0.0, -1.0), b: Vec2::new(0.0, 1.0), stroke: white_stroke(1.0) }),
        // Score text
        DrawCmd::Text {
            pos: Vec2::new(-0.3, 0.8),
            text: "3".to_string(),
            size_px: 32.0,
            color: Rgba::WHITE,
            style: FontStyleId::ATARI,
        },
        DrawCmd::Text {
            pos: Vec2::new(0.2, 0.8),
            text: "2".to_string(),
            size_px: 32.0,
            color: Rgba::WHITE,
            style: FontStyleId::ATARI,
        },
    ];

    let stats = renderer.render(&cmds);
    assert_eq!(stats.lines, 1, "should have 1 center line");
    assert_eq!(stats.polylines, 3, "should have 3 polylines (paddles + ball)");
    assert_eq!(stats.text_runs, 2, "should have 2 score texts");
}

#[test]
fn renders_asteroids_like_scene() {
    let mut renderer = NullRenderer;
    let cmds = vec![
        DrawCmd::Clear { color: Rgba::BLACK },
        // Ship (triangle)
        DrawCmd::PushTransform(Mat3::from_rotation_z(0.5)),
        DrawCmd::Polyline {
            pts: vec![Vec2::new(0.0, 0.05), Vec2::new(-0.03, -0.03), Vec2::new(0.03, -0.03)],
            closed: true,
            stroke: white_stroke(1.5),
        },
        DrawCmd::PopTransform,
        // Asteroid 1 (irregular polygon)
        DrawCmd::PushTransform(Mat3::from_translation(Vec2::new(0.5, 0.3))),
        DrawCmd::Polyline {
            pts: vec![
                Vec2::new(0.0, 0.1), Vec2::new(0.08, 0.06), Vec2::new(0.1, 0.0),
                Vec2::new(0.07, -0.08), Vec2::new(-0.05, -0.1), Vec2::new(-0.1, -0.02),
                Vec2::new(-0.08, 0.07),
            ],
            closed: true,
            stroke: white_stroke(1.5),
        },
        DrawCmd::PopTransform,
        // Bullet
        DrawCmd::Line(Line2 {
            a: Vec2::new(0.1, 0.1),
            b: Vec2::new(0.12, 0.12),
            stroke: white_stroke(2.0),
        }),
        // Score
        DrawCmd::Text {
            pos: Vec2::new(-0.9, 0.9),
            text: "1250".to_string(),
            size_px: 24.0,
            color: Rgba::WHITE,
            style: FontStyleId::ATARI,
        },
    ];

    let stats = renderer.render(&cmds);
    assert_eq!(stats.lines, 1, "should have 1 bullet line");
    assert_eq!(stats.polylines, 2, "should have 2 polylines (ship + asteroid)");
    assert_eq!(stats.text_runs, 1, "should have 1 score text");
}

#[test]
fn handles_nested_transforms() {
    let mut renderer = NullRenderer;
    let cmds = vec![
        DrawCmd::Clear { color: Rgba::BLACK },
        DrawCmd::PushTransform(Mat3::from_translation(Vec2::new(0.5, 0.0))),
        DrawCmd::PushTransform(Mat3::from_scale(Vec2::new(0.5, 0.5))),
        DrawCmd::PushTransform(Mat3::from_rotation_z(std::f32::consts::PI / 4.0)),
        DrawCmd::Line(Line2 { a: Vec2::ZERO, b: Vec2::new(0.1, 0.0), stroke: white_stroke(1.0) }),
        DrawCmd::PopTransform,
        DrawCmd::PopTransform,
        DrawCmd::PopTransform,
    ];

    let stats = renderer.render(&cmds);
    assert_eq!(stats.lines, 1);
}

#[test]
fn handles_all_font_styles() {
    let mut renderer = NullRenderer;
    let cmds = vec![
        DrawCmd::Clear { color: Rgba::BLACK },
        DrawCmd::Text { pos: Vec2::new(-0.8, 0.6), text: "ATARI".to_string(), size_px: 20.0, color: Rgba::WHITE, style: FontStyleId::ATARI },
        DrawCmd::Text { pos: Vec2::new(-0.8, 0.3), text: "CINEMA".to_string(), size_px: 20.0, color: Rgba::WHITE, style: FontStyleId::CINEMATRONICS },
        DrawCmd::Text { pos: Vec2::new(-0.8, 0.0), text: "MIDWAY".to_string(), size_px: 20.0, color: Rgba::WHITE, style: FontStyleId::MIDWAY },
        DrawCmd::Text { pos: Vec2::new(-0.8, -0.3), text: "SCAN".to_string(), size_px: 20.0, color: Rgba::WHITE, style: FontStyleId::VECTOR_SCANLINE },
    ];

    let stats = renderer.render(&cmds);
    assert_eq!(stats.text_runs, 4, "should render all 4 font styles");
}

#[test]
fn handles_colored_strokes() {
    let mut renderer = NullRenderer;
    let green = Rgba(0.0, 1.0, 0.0, 1.0);
    let red = Rgba(1.0, 0.0, 0.0, 1.0);
    let blue = Rgba(0.0, 0.0, 1.0, 1.0);

    let cmds = vec![
        DrawCmd::Clear { color: Rgba::BLACK },
        DrawCmd::Line(Line2 { a: Vec2::new(-0.5, 0.0), b: Vec2::new(0.5, 0.0), stroke: Stroke::new(green, 3.0) }),
        DrawCmd::Line(Line2 { a: Vec2::new(0.0, -0.5), b: Vec2::new(0.0, 0.5), stroke: Stroke::new(red, 3.0) }),
        DrawCmd::Polyline {
            pts: vec![Vec2::new(-0.3, -0.3), Vec2::new(0.3, -0.3), Vec2::new(0.0, 0.3)],
            closed: true,
            stroke: Stroke::new(blue, 2.0),
        },
    ];

    let stats = renderer.render(&cmds);
    assert_eq!(stats.lines, 2);
    assert_eq!(stats.polylines, 1);
}
