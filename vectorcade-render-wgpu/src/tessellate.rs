//! Line and polyline tessellation using lyon.
//!
//! Converts vector primitives into triangle meshes for GPU rendering.

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
pub fn tessellate_line(line: &Line2, geom: &mut Geometry) {
    tessellate_stroke(&[line.a.into(), line.b.into()], false, &line.stroke, geom);
}

/// Tessellate a polyline into triangles.
pub fn tessellate_polyline(pts: &[[f32; 2]], closed: bool, stroke: &Stroke, geom: &mut Geometry) {
    tessellate_stroke(pts, closed, stroke, geom);
}

fn tessellate_stroke(pts: &[[f32; 2]], closed: bool, stroke: &Stroke, geom: &mut Geometry) {
    if pts.len() < 2 {
        return;
    }

    let path = build_path(pts, closed);
    let options = stroke_options(stroke.width_px);
    let color = [
        stroke.color.0,
        stroke.color.1,
        stroke.color.2,
        stroke.color.3,
    ];

    let base_vertex = geom.vertices.len() as u32;
    let mut buffers: VertexBuffers<Vertex, u32> = VertexBuffers::new();
    let mut tessellator = StrokeTessellator::new();

    let result = tessellator.tessellate_path(
        &path,
        &options,
        &mut BuffersBuilder::new(&mut buffers, |v: lyon::tessellation::StrokeVertex| Vertex {
            position: [v.position().x, v.position().y],
            color,
        }),
    );

    if result.is_ok() {
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

fn stroke_options(width: f32) -> StrokeOptions {
    StrokeOptions::default()
        .with_line_width(width)
        .with_line_cap(LineCap::Round)
        .with_line_join(LineJoin::Round)
}
