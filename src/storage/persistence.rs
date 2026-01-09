use std::fs;
use std::io;
use std::path::PathBuf;

use crate::app::App;
use crate::ui::colors::create_palette;

/// Get the save file path
pub fn get_save_path() -> io::Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find data directory"))?;

    let app_dir = data_dir.join("ganjatui");

    // Create directory if it doesn't exist
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
    }

    Ok(app_dir.join("save.json"))
}

/// Save application state to disk
pub fn save(app: &App) -> io::Result<()> {
    let path = get_save_path()?;
    let json = serde_json::to_string_pretty(app)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    fs::write(path, json)?;
    Ok(())
}

/// Load application state from disk
pub fn load(supports_truecolor: bool) -> io::Result<App> {
    let path = get_save_path()?;

    if !path.exists() {
        // No save file, return default app with a new plant
        return Ok(App::new(supports_truecolor));
    }

    let json = fs::read_to_string(path)?;
    let mut app: App = serde_json::from_str(&json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Restore UI state
    app.running = true;
    app.current_screen = crate::message::Screen::GrowingRoom;
    app.animation_frame = 0;
    app.color_palette = create_palette(supports_truecolor, app.visual_mode);

    Ok(app)
}

/// Delete save file (for testing)
#[allow(dead_code)]
pub fn delete_save() -> io::Result<()> {
    let path = get_save_path()?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}
