# Product Requirements Document (PRD)

## Product Overview

vectorcade-render-wgpu provides GPU-accelerated rendering for classic vector-style arcade games. It enables the VectorCade platform to display crisp, scalable vector graphics with authentic retro aesthetics.

## Goals

1. **Render classic vector game visuals**: Lines, polylines, and text with configurable stroke width and color
2. **Cross-platform support**: Run in browsers (WebGPU/WASM) and natively (Vulkan/Metal/DX12)
3. **Performance**: Handle 200+ vector elements at 60fps
4. **Authentic aesthetics**: Support phosphor glow effects for CRT-style appearance

## Target Games

The renderer must support the visual requirements of:
- Pong (simple lines, rectangles)
- Asteroids (many polylines, explosions)
- Lunar Lander (terrain polylines, ship, particles)
- Battlezone (3D wireframe projected to 2D)
- Tempest (complex tube geometry)

## Functional Requirements

### FR-1: Draw Commands

Support rendering these `DrawCmd` types:
- `Clear { color }` - Fill background
- `Line { a, b, stroke }` - Single line segment
- `Polyline { pts, closed, stroke }` - Connected line segments
- `Text { pos, s, size_px, color }` - Rendered via vector fonts

### FR-2: Stroke Attributes

Each stroke supports:
- `color: Rgba` - RGBA color (f32 components)
- `width_px: f32` - Stroke width in screen pixels
- `glow: f32` - Optional phosphor glow intensity

### FR-3: Consistent Thick Lines

Lines must render consistently across all platforms (no platform-specific line width quirks).

### FR-4: Transform Stack

Support `PushTransform` and `PopTransform` for hierarchical rendering.

## Non-Functional Requirements

### NFR-1: Performance

- Render 500 line segments at 60fps on mid-range hardware
- Minimize per-frame allocations

### NFR-2: Portability

- Compile to WASM without platform-specific code
- Work on Chrome, Firefox, Safari (WebGPU-enabled)
- Work natively on macOS, Windows, Linux

### NFR-3: Testability

- `NullRenderer` for headless tests
- Render stats for verification

## Out of Scope (Phase 1)

- Filled shapes (only stroked paths)
- Bitmap/texture rendering
- Audio
- 3D perspective projection (handled by games using shared helpers)

## Success Metrics

1. All VectorCade games render correctly in browser and native
2. No visual differences between platforms
3. Consistent 60fps at target scene complexity
