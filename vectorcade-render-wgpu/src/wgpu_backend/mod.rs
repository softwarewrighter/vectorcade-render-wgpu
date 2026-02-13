//! wgpu-based GPU renderer for VectorCade.
//!
//! This module provides the `WgpuRenderer` which renders `DrawCmd` display-lists
//! using wgpu (WebGPU in browser, Vulkan/Metal/DX12 native).

mod pipeline;
mod state;

use crate::tessellate::{Geometry, tessellate_line, tessellate_polyline};
use crate::{RenderStats, VectorRenderer};
use state::RenderState;
use vectorcade_shared::draw::DrawCmd;
use wgpu::util::DeviceExt;

/// GPU renderer using wgpu.
pub struct WgpuRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    geometry: Geometry,
    state: RenderState,
}

impl WgpuRenderer {
    /// Create a new renderer for the given window.
    ///
    /// # Errors
    /// Returns an error if GPU initialization fails.
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window).map_err(|e| e.to_string())?;
        let (device, queue, config) = pipeline::init_device(&instance, &surface, width, height).await?;
        let surface_format = config.format;
        surface.configure(&device, &config);
        let pipeline = pipeline::create(&device, surface_format);

        Ok(Self {
            device,
            queue,
            pipeline,
            surface,
            config,
            geometry: Geometry::new(),
            state: RenderState::default(),
        })
    }

    /// Resize the render surface.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

impl VectorRenderer for WgpuRenderer {
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats {
        self.state.reset();
        let stats = self.tessellate_commands(cmds);

        let Some(frame) = self.begin_frame() else {
            return stats;
        };

        self.draw_frame(&frame);
        frame.output.present();
        stats
    }
}

impl WgpuRenderer {
    fn tessellate_commands(&mut self, cmds: &[DrawCmd]) -> RenderStats {
        let mut stats = RenderStats::default();
        self.geometry.clear();

        for cmd in cmds {
            match cmd {
                DrawCmd::Clear { color } => self.state.clear = *color,
                DrawCmd::PushTransform(t) => self.state.push(*t),
                DrawCmd::PopTransform => self.state.pop(),
                DrawCmd::Line(line) => {
                    let t = self.state.transform_opt();
                    tessellate_line(line, t.as_ref(), &mut self.geometry);
                    stats.lines += 1;
                }
                DrawCmd::Polyline {
                    pts,
                    closed,
                    stroke,
                } => {
                    let t = self.state.transform_opt();
                    let pts: Vec<[f32; 2]> = pts.iter().map(|v| [v.x, v.y]).collect();
                    tessellate_polyline(&pts, *closed, stroke, t.as_ref(), &mut self.geometry);
                    stats.polylines += 1;
                }
                DrawCmd::Text { .. } => stats.text_runs += 1,
                _ => {}
            }
        }
        stats
    }

    fn begin_frame(&self) -> Option<Frame> {
        let output = self.surface.get_current_texture().ok()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Some(Frame { output, view })
    }

    fn draw_frame(&self, frame: &Frame) {
        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.geometry.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.geometry.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.state.clear_color()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.geometry.indices.len() as u32, 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}

struct Frame {
    output: wgpu::SurfaceTexture,
    view: wgpu::TextureView,
}
