# vectorcade-render-wgpu

WGPU backend that renders VectorCade `DrawCmd` display-lists.

Plan:
- Convert `DrawCmd::Line` and `DrawCmd::Polyline` into triangle meshes (thick strokes)
  using `lyon` tessellation.
- Render via `wgpu` (WebGPU in browser; native on desktop).
- Optional phosphor glow via postprocess or multi-pass (later).

This repo depends on `vectorcade-shared` only.

## Build (native)

```bash
cargo test
```

WASM build will be exercised from `vectorcade-web-yew`.
