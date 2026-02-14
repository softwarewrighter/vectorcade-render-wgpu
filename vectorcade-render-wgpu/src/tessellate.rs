//! Line and polyline tessellation using lyon.
//!
//! Converts vector primitives into triangle meshes for GPU rendering.

use glam::Mat3;
use lyon::math::Point;
use lyon::path::Path;
use lyon::tessellation::{
    BuffersBuilder, LineCap, LineJoin, StrokeOptions, StrokeTessellator, VertexBuffers,
};
use vectorcade_shared::draw::{Line2, Stroke};

/// A vertex with position and color for GPU rendering.
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

/// Tessellated geometry ready for GPU upload.
#[derive(Clone, Debug, Default)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Geometry {
    /// Create empty geometry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all vertices and indices.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

/// Tessellate a single line segment into triangles.
///
/// `px_to_ndc` converts pixel measurements to NDC units (typically `2.0 / viewport_height`).
pub fn tessellate_line(line: &Line2, transform: Option<&Mat3>, px_to_ndc: f32, geom: &mut Geometry) {
    let xform = |p: [f32; 2], t: &Mat3| {
        let r = *t * glam::Vec3::new(p[0], p[1], 1.0);
        [r.x, r.y]
    };
    let (a, b): ([f32; 2], [f32; 2]) = match transform {
        Some(t) => (xform(line.a.into(), t), xform(line.b.into(), t)),
        None => (line.a.into(), line.b.into()),
    };
    tessellate_stroke(&[a, b], false, &line.stroke, px_to_ndc, geom);
}

/// Tessellate a polyline into triangles.
///
/// `px_to_ndc` converts pixel measurements to NDC units (typically `2.0 / viewport_height`).
pub fn tessellate_polyline(
    pts: &[[f32; 2]],
    closed: bool,
    stroke: &Stroke,
    transform: Option<&Mat3>,
    px_to_ndc: f32,
    geom: &mut Geometry,
) {
    let pts: Vec<[f32; 2]> = match transform {
        Some(t) => pts
            .iter()
            .map(|p| {
                let r = *t * glam::Vec3::new(p[0], p[1], 1.0);
                [r.x, r.y]
            })
            .collect(),
        None => pts.to_vec(),
    };
    tessellate_stroke(&pts, closed, stroke, px_to_ndc, geom);
}

fn tessellate_stroke(pts: &[[f32; 2]], closed: bool, stroke: &Stroke, px_to_ndc: f32, geom: &mut Geometry) {
    if pts.len() < 2 {
        return;
    }

    let path = build_path(pts, closed);
    // Convert pixel width to NDC units
    let width_ndc = stroke.width_px * px_to_ndc;
    let options = StrokeOptions::default()
        .with_line_width(width_ndc)
        .with_line_cap(LineCap::Round)
        .with_line_join(LineJoin::Round);
    let color = [
        stroke.color.0,
        stroke.color.1,
        stroke.color.2,
        stroke.color.3,
    ];

    let base_vertex = geom.vertices.len() as u32;
    let mut buffers: VertexBuffers<Vertex, u32> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    if tessellator
        .tessellate_path(
            &path,
            &options,
            &mut BuffersBuilder::new(&mut buffers, |v: lyon::tessellation::StrokeVertex| Vertex {
                position: [v.position().x, v.position().y],
                color,
            }),
        )
        .is_ok()
    {
        geom.vertices.extend(buffers.vertices);
        geom.indices
            .extend(buffers.indices.iter().map(|i| i + base_vertex));
    }
}

fn build_path(pts: &[[f32; 2]], closed: bool) -> Path {
    let mut builder = Path::builder();
    builder.begin(Point::new(pts[0][0], pts[0][1]));
    for p in &pts[1..] {
        builder.line_to(Point::new(p[0], p[1]));
    }
    if closed {
        builder.close();
    } else {
        builder.end(false);
    }
    builder.build()
}
