//! Renderer backend (wgpu + lyon) for VectorCade.
//!
//! This skeleton provides the public API shape; implementation is Agent-owned.

use vectorcade_shared::draw::DrawCmd;

#[derive(Clone, Copy, Debug)]
pub struct RenderStats {
    pub lines: u32,
    pub polylines: u32,
    pub text_runs: u32,
}

pub trait VectorRenderer {
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats;
}

/// Placeholder "null" renderer for tests and headless runs.
pub struct NullRenderer;

impl VectorRenderer for NullRenderer {
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats {
        let mut s = RenderStats {
            lines: 0,
            polylines: 0,
            text_runs: 0,
        };
        for c in cmds {
            match c {
                DrawCmd::Line(_) => s.lines += 1,
                DrawCmd::Polyline { .. } => s.polylines += 1,
                DrawCmd::Text { .. } => s.text_runs += 1,
                _ => {}
            }
        }
        s
    }
}
