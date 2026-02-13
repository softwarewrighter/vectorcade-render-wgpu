# Architecture

## Overview

vectorcade-render-wgpu is the GPU rendering backend for the VectorCade vector graphics game platform. It converts abstract `DrawCmd` display-lists into GPU-rendered output using wgpu and lyon.

## System Context

This crate is part of a multi-repo DAG:

```
vectorcade-shared (root)
    |
    +-- vectorcade-fonts (depends on shared)
    |
    +-- vectorcade-render-wgpu (depends on shared) <-- THIS REPO
    |
    +-- vectorcade-games (depends on shared + fonts)
    |
    +-- vectorcade-web-yew (depends on shared + games + render-wgpu)
```

## Core Components

### VectorRenderer Trait

The primary abstraction for rendering backends:

```rust
pub trait VectorRenderer {
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats;
}
```

Games produce `DrawCmd` display-lists; renderers consume them. This decoupling allows:
- Headless testing with `NullRenderer`
- Native rendering with wgpu
- WASM/WebGPU rendering in browsers

### Rendering Pipeline (Planned)

```
DrawCmd --> Lyon Tessellation --> Triangle Mesh --> wgpu Pipeline --> GPU
```

1. **Input**: `DrawCmd` display-list (lines, polylines, text)
2. **Tessellation**: Lyon converts thick strokes to triangle geometry
3. **Upload**: Vertex/index buffers uploaded to GPU
4. **Render**: wgpu pipeline renders triangles with color

### Key Design Decisions

1. **Triangle-based thick lines**: Wide lines are inconsistent across platforms in WebGL/WebGPU. Lyon tessellates stroked paths into triangles for reliable rendering.

2. **Display-list model**: Games never touch wgpu directly. They output `DrawCmd`s, which are rendered by this backend.

3. **Normalized world space**: Games render to normalized coordinates (e.g., [-1..1]). The renderer maps to actual viewport.

4. **Optional phosphor glow**: Post-processing for CRT-style bloom effects (future).

## Dependencies

- `vectorcade-shared`: Core types (`DrawCmd`, `Rgba`, `Stroke`, etc.)
- `wgpu`: GPU abstraction (WebGPU in browser, Vulkan/Metal/DX12 native)
- `lyon`: 2D path tessellation

## Build Targets

- Native: Standard Rust compilation
- WASM: Built via `wasm-pack`, consumed by `vectorcade-web-yew`
