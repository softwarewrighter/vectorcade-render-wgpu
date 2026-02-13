//! Render statistics tracking.

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
