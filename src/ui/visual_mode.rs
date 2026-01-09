use serde::{Deserialize, Serialize};

/// Visual modes for different aesthetic themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualMode {
    /// Current RGB/256/16 color system (default)
    Normal,
    /// Zen Garden - minimalist, soft colors, slow breathing
    Zen,
    /// Rainbow/Psychedelic - HSV cycling colors, energetic
    Rainbow,
    /// Matrix - green monochrome, retro hacker aesthetic
    Matrix,
}

impl VisualMode {
    /// Cycle to the next visual mode
    pub fn next(&self) -> Self {
        match self {
            VisualMode::Normal => VisualMode::Zen,
            VisualMode::Zen => VisualMode::Rainbow,
            VisualMode::Rainbow => VisualMode::Matrix,
            VisualMode::Matrix => VisualMode::Normal,
        }
    }

    /// Get the display name of the mode
    pub fn name(&self) -> &'static str {
        match self {
            VisualMode::Normal => "Normal",
            VisualMode::Zen => "Zen Garden",
            VisualMode::Rainbow => "Rainbow",
            VisualMode::Matrix => "Matrix",
        }
    }
}

impl Default for VisualMode {
    fn default() -> Self {
        VisualMode::Normal
    }
}
