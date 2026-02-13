# Design Document

## Overview

This document describes the technical design for vectorcade-render-wgpu.

## Module Structure

```
vectorcade-render-wgpu/
+-- src/
    +-- lib.rs          # Public API, VectorRenderer trait
    +-- null.rs         # NullRenderer (headless)
    +-- wgpu_backend/   # wgpu implementation (feature-gated)
        +-- mod.rs      # WgpuRenderer struct
        +-- pipeline.rs # Shader pipeline setup
        +-- tessellate.rs # Lyon tessellation
        +-- buffers.rs  # Vertex/index buffer management
```

## VectorRenderer Trait

```rust
pub trait VectorRenderer {
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats;
}
```

Implementations:
- `NullRenderer`: Counts commands, no GPU (for tests)
- `WgpuRenderer`: Full GPU rendering (feature: `wgpu-backend`)

## WgpuRenderer Design

### Initialization

```rust
impl WgpuRenderer {
    pub async fn new(window: &Window) -> Self { ... }
}
```

Creates:
- `wgpu::Instance`, `Adapter`, `Device`, `Queue`
- Render pipeline with vertex/fragment shaders
- Vertex buffer pool

### Render Loop

1. **Collect**: Iterate `DrawCmd` list
2. **Tessellate**: For each Line/Polyline, use Lyon to generate triangles
3. **Upload**: Write vertices to GPU buffer
4. **Draw**: Submit draw calls

### Vertex Format

```rust
#[repr(C)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}
```

All geometry converted to colored triangles.

## Lyon Tessellation

Using `lyon::tessellation::StrokeTessellator`:

```rust
let mut tessellator = StrokeTessellator::new();
let options = StrokeOptions::default()
    .with_line_width(stroke.width_px)
    .with_line_cap(LineCap::Round)
    .with_line_join(LineJoin::Round);

tessellator.tessellate_path(&path, &options, &mut vertex_builder)?;
```

## Shader Design

### Vertex Shader

```wgsl
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}
```

### Fragment Shader

```wgsl
@fragment
fn fs_main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
```

## Transform Stack

Maintain a stack of 2D affine transforms:

```rust
struct RenderState {
    transform_stack: Vec<Transform2>,
    current_transform: Transform2,
}
```

On `PushTransform`: push current, multiply with new
On `PopTransform`: restore from stack

Apply transform to vertices before tessellation.

## Phosphor Glow (Future)

Options:
1. Multi-pass: Render lines, blur, composite
2. Wider soft-edge triangles with gradient alpha
3. Post-process bloom shader

Deferred to Phase 2.

## Error Handling

- Missing GPU: Return error from `WgpuRenderer::new()`
- Invalid commands: Skip with warning, continue rendering
- Buffer overflow: Dynamically resize or flush

## Testing Strategy

- Unit tests: `NullRenderer` counts commands correctly
- Visual tests: Render known scene, compare screenshots (manual)
- Integration: End-to-end with `vectorcade-games` via `vectorcade-web-yew`
