pub mod colors;
pub mod growing;
pub mod layout;
pub mod stats;
pub mod visual_mode;

use ratatui::Frame;

use crate::app::App;
use crate::message::Screen;

/// Main view function - renders the current screen
pub fn view(f: &mut Frame, app: &App) {
    let area = f.area();

    match app.current_screen {
        Screen::GrowingRoom => growing::render(f, app, area),
        Screen::Stats => stats::render(f, app, area),
    }
}
