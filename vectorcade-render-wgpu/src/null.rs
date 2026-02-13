//! Null renderer for testing and headless runs.

use crate::{RenderStats, VectorRenderer};
use vectorcade_shared::draw::DrawCmd;

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
