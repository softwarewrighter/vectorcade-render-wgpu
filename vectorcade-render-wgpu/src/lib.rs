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
//! - [`wgpu_backend`] - GPU renderer (requires `wgpu-backend` feature)

mod null;
mod renderer;
mod stats;
pub mod tessellate;

#[cfg(feature = "wgpu-backend")]
pub mod wgpu_backend;

pub use null::NullRenderer;
pub use renderer::VectorRenderer;
pub use stats::RenderStats;
pub use tessellate::{Geometry, Vertex};

#[cfg(feature = "wgpu-backend")]
pub use wgpu_backend::WgpuRenderer;
