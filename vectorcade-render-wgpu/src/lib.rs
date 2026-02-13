//! Renderer backend (wgpu + lyon) for VectorCade.
//!
//! This crate provides GPU-accelerated rendering for VectorCade games.
//! It converts `DrawCmd` display-lists
//! into rendered output using wgpu and lyon tessellation.
//!
//! # Features
//!
//! - **Line tessellation** via lyon with round caps/joins
//! - **Transform stack** for hierarchical rendering
//! - **Text rendering** via vectorcade-fonts (Atari, Cinematronics, Midway styles)
//! - **4x MSAA** anti-aliasing for smooth edges
//! - **WASM compatible** for WebGPU in browsers
//!
//! # Usage
//!
//! ```ignore
//! use vectorcade_render_wgpu::{WgpuRenderer, VectorRenderer};
//!
//! // Create renderer (requires window handle)
//! let mut renderer = WgpuRenderer::new(window, width, height).await?;
//!
//! // Render draw commands from a game
//! let stats = renderer.render(&draw_commands);
//! println!("Rendered {} lines, {} polylines", stats.lines, stats.polylines);
//! ```
//!
//! # Feature Flags
//!
//! - `wgpu-backend` - Enables GPU rendering via wgpu (includes vectorcade-fonts)
//!
//! # Modules
//!
//! - [`tessellate`] - Line tessellation using lyon
//! - [`wgpu_backend`] - GPU renderer (requires `wgpu-backend` feature)

pub mod tessellate;

#[cfg(feature = "wgpu-backend")]
pub mod wgpu_backend;

use vectorcade_shared::draw::DrawCmd;

/// Statistics about rendered primitives.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderStats {
    /// Number of individual line segments rendered.
    pub lines: u32,
    /// Number of polyline paths rendered.
    pub polylines: u32,
    /// Number of text runs rendered.
    pub text_runs: u32,
}

/// Trait for vector graphics renderers.
///
/// Implementations consume `DrawCmd` display-lists and produce rendered output.
pub trait VectorRenderer {
    /// Render a list of draw commands and return statistics.
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats;
}

/// Placeholder renderer for tests and headless runs.
///
/// Counts draw commands without producing any visual output.
pub struct NullRenderer;

impl VectorRenderer for NullRenderer {
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats {
        let mut stats = RenderStats::default();
        for cmd in cmds {
            match cmd {
                DrawCmd::Line(_) => stats.lines += 1,
                DrawCmd::Polyline { .. } => stats.polylines += 1,
                DrawCmd::Text { .. } => stats.text_runs += 1,
                _ => {}
            }
        }
        stats
    }
}

pub use tessellate::{Geometry, Vertex};

#[cfg(feature = "wgpu-backend")]
pub use wgpu_backend::WgpuRenderer;
