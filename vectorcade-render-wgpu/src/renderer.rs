//! Vector renderer trait definition.

use crate::RenderStats;
use vectorcade_shared::draw::DrawCmd;

/// Trait for vector graphics renderers.
///
/// Implementations consume `DrawCmd` display-lists and produce rendered output.
pub trait VectorRenderer {
    /// Render a list of draw commands and return statistics.
    fn render(&mut self, cmds: &[DrawCmd]) -> RenderStats;
}
