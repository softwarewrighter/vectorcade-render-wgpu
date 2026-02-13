//! Renderer backend (wgpu + lyon) for VectorCade.
//!
//! This crate provides GPU-accelerated rendering for VectorCade games.
//! It converts `DrawCmd` display-lists into rendered output using wgpu and lyon.
//!
//! # Modules
//!
//! - [`null`] - Null renderer for testing and headless runs
//! - [`renderer`] - VectorRenderer trait definition
//! - [`stats`] - Render statistics
//! - [`tessellate`] - Line tessellation using lyon

mod null;
mod renderer;
mod stats;
pub mod tessellate;

pub use null::NullRenderer;
pub use renderer::VectorRenderer;
pub use stats::RenderStats;
pub use tessellate::{Geometry, Vertex};
