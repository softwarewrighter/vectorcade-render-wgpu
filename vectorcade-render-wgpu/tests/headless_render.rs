//! Headless rendering test - renders to texture and reads back pixels.
//!
//! This test verifies GPU rendering produces visible output without needing a window.

#![cfg(feature = "wgpu-backend")]

use glam::Vec2;
use vectorcade_render_wgpu::tessellate::{tessellate_line, tessellate_polyline, Geometry};
use vectorcade_shared::draw::{Line2, Stroke};
use vectorcade_shared::Rgba;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

/// Render a simple scene to a texture and verify pixels are non-black.
#[test]
fn headless_render_produces_visible_output() {
    pollster::block_on(async {
        // 1. Setup wgpu
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("No adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .expect("No device");

        // 2. Create render target texture
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Target"),
            size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 3. Create simple pipeline (same as WgpuRenderer but without MSAA for simplicity)
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Test Shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Test Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 24, // 2 floats pos + 4 floats color
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 },
                        wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                    ],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // 4. Tessellate a white triangle in the center
        let mut geom = Geometry::new();

        // Draw a large white triangle
        let stroke = Stroke::new(Rgba::WHITE, 0.05); // stroke width in NDC
        tessellate_polyline(
            &[
                [0.0, 0.5],   // top
                [-0.5, -0.5], // bottom-left
                [0.5, -0.5],  // bottom-right
            ],
            true, // closed
            &stroke,
            None,
            &mut geom,
        );

        // Also draw some lines
        tessellate_line(
            &Line2 {
                a: Vec2::new(-0.8, 0.0),
                b: Vec2::new(0.8, 0.0),
                stroke: Stroke::new(Rgba(1.0, 0.0, 0.0, 1.0), 0.02), // red horizontal line
            },
            None,
            &mut geom,
        );

        println!("Tessellated {} vertices, {} indices", geom.vertices.len(), geom.indices.len());
        assert!(!geom.vertices.is_empty(), "Tessellation should produce vertices");
        assert!(!geom.indices.is_empty(), "Tessellation should produce indices");

        // 5. Create GPU buffers
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (geom.vertices.len() * 24) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&geom.vertices));

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: (geom.indices.len() * 4) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(&geom.indices));

        // 6. Render
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Test Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&pipeline);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..geom.indices.len() as u32, 0, 0..1);
        }

        // 7. Copy texture to buffer for readback
        let bytes_per_row = 4 * WIDTH; // RGBA
        let padded_bytes_per_row = (bytes_per_row + 255) & !255; // align to 256
        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (padded_bytes_per_row * HEIGHT) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(HEIGHT),
                },
            },
            wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        );

        queue.submit(Some(encoder.finish()));

        // 8. Read back pixels
        let buffer_slice = output_buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().expect("Failed to map buffer");

        let data = buffer_slice.get_mapped_range();

        // Count non-black pixels
        let mut non_black_count = 0;
        let mut sample_pixels = Vec::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let offset = (y * padded_bytes_per_row + x * 4) as usize;
                let r = data[offset];
                let g = data[offset + 1];
                let b = data[offset + 2];
                if r > 0 || g > 0 || b > 0 {
                    non_black_count += 1;
                    if sample_pixels.len() < 10 {
                        sample_pixels.push((x, y, r, g, b));
                    }
                }
            }
        }

        println!("Non-black pixels: {} / {}", non_black_count, WIDTH * HEIGHT);
        println!("Sample pixels: {:?}", sample_pixels);

        assert!(non_black_count > 100, "Should have visible rendered content (got {} non-black pixels)", non_black_count);
    });
}

const SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
"#;
