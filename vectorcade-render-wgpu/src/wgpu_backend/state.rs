//! Render state for transform stack and clear color.

use glam::Mat3;
use vectorcade_shared::Rgba;

/// Render state for transform stack and clear color.
#[derive(Debug, Default)]
pub struct RenderState {
    transform_stack: Vec<Mat3>,
    current: Mat3,
    pub clear: Rgba,
}

impl RenderState {
    pub fn reset(&mut self) {
        self.transform_stack.clear();
        self.current = Mat3::IDENTITY;
        self.clear = Rgba::BLACK;
    }

    pub fn push(&mut self, t: Mat3) {
        self.transform_stack.push(self.current);
        self.current *= t;
    }

    pub fn pop(&mut self) {
        if let Some(t) = self.transform_stack.pop() {
            self.current = t;
        }
    }

    pub fn transform_opt(&self) -> Option<Mat3> {
        (self.current != Mat3::IDENTITY).then_some(self.current)
    }

    pub fn clear_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.clear.0 as f64,
            g: self.clear.1 as f64,
            b: self.clear.2 as f64,
            a: self.clear.3 as f64,
        }
    }
}
