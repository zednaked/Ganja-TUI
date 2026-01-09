/// Layout modes based on terminal size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// Small terminals (<80 cols or <40 rows) - vertical stacking
    Small,
    /// Medium terminals (80-120 cols) - current 70/30 horizontal split
    Medium,
    /// Large terminals (>120 cols) - centered plant with side panels
    Large,
}

impl LayoutMode {
    /// Determine layout mode from terminal dimensions
    pub fn from_terminal_size(width: u16, height: u16) -> Self {
        if width < 80 || height < 40 {
            LayoutMode::Small
        } else if width <= 120 {
            LayoutMode::Medium
        } else {
            LayoutMode::Large
        }
    }

    /// Get short indicator for UI display
    pub fn indicator(&self) -> &'static str {
        match self {
            LayoutMode::Small => "S",
            LayoutMode::Medium => "M",
            LayoutMode::Large => "L",
        }
    }
}
