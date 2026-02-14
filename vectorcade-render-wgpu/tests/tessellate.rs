//! Tests for tessellation module.

use glam::Vec2;
use vectorcade_render_wgpu::tessellate::{Geometry, tessellate_line, tessellate_polyline};
use vectorcade_shared::Rgba;
use vectorcade_shared::draw::{Line2, Stroke};

fn white_stroke(width: f32) -> Stroke {
    Stroke::new(Rgba::WHITE, width)
}

// Use 1.0 for px_to_ndc in tests (treats stroke width as already in NDC)
const PX_TO_NDC: f32 = 1.0;

#[test]
fn tessellate_line_produces_geometry() {
    let line = Line2 {
        a: Vec2::ZERO,
        b: Vec2::new(1.0, 0.0),
        stroke: white_stroke(0.1), // use NDC-scale width for test
    };
    let mut geom = Geometry::new();
    tessellate_line(&line, None, PX_TO_NDC, &mut geom);

    assert!(!geom.vertices.is_empty(), "should produce vertices");
    assert!(!geom.indices.is_empty(), "should produce indices");
    assert_eq!(geom.indices.len() % 3, 0, "indices should form triangles");
}

#[test]
fn tessellate_polyline_produces_geometry() {
    let pts = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]];
    let mut geom = Geometry::new();
    tessellate_polyline(&pts, false, &white_stroke(0.1), None, PX_TO_NDC, &mut geom);

    assert!(!geom.vertices.is_empty());
    assert!(!geom.indices.is_empty());
}

#[test]
fn tessellate_closed_polyline() {
    let pts = [[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]];
    let mut geom = Geometry::new();
    tessellate_polyline(&pts, true, &white_stroke(0.1), None, PX_TO_NDC, &mut geom);

    assert!(!geom.vertices.is_empty());
}

#[test]
fn empty_points_produces_no_geometry() {
    let mut geom = Geometry::new();
    tessellate_polyline(&[], false, &white_stroke(0.1), None, PX_TO_NDC, &mut geom);
    assert!(geom.vertices.is_empty());
}

#[test]
fn single_point_produces_no_geometry() {
    let mut geom = Geometry::new();
    tessellate_polyline(&[[0.0, 0.0]], false, &white_stroke(0.1), None, PX_TO_NDC, &mut geom);
    assert!(geom.vertices.is_empty());
}
