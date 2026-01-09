mod app;
mod ascii;
mod domain;
mod message;
mod storage;
mod ui;
mod update;

use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use message::{Message, Screen};
use update::update;

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Detect terminal color capabilities
    let supports_truecolor = supports_color::on(supports_color::Stream::Stdout)
        .map(|level| level.has_16m)
        .unwrap_or(false);

    // Load or create app state
    let mut app = storage::load(supports_truecolor).unwrap_or_else(|_| App::new(supports_truecolor));

    // Run the main loop
    let result = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Print any errors
    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // 1. RENDER: Draw the current state
        terminal.draw(|f| ui::view(f, app))?;

        // 2. INPUT: Poll for events with timeout (50ms for smooth animations)
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Only process KeyPress events (ignore KeyRelease)
                if key.kind == KeyEventKind::Press {
                    let message = key_to_message(key, app);

                    // 3. UPDATE: Transform state based on message
                    *app = update(app.clone(), message);

                    // 4. PERSIST: Save state after updates
                    if let Err(e) = storage::save(app) {
                        eprintln!("Failed to save: {}", e);
                    }

                    // Check if we should quit
                    if !app.running {
                        break;
                    }
                }
            }
        } else {
            // No input received, send Tick message for time updates
            *app = update(app.clone(), Message::Tick);

            // Save periodically (every tick)
            if let Err(e) = storage::save(app) {
                eprintln!("Failed to save: {}", e);
            }
        }
    }

    Ok(())
}

/// Convert keyboard input to messages
fn key_to_message(key: KeyEvent, app: &App) -> Message {
    match key.code {
        // Global keys
        KeyCode::Char('q') => Message::Quit,
        KeyCode::Char('1') => Message::SwitchScreen(Screen::GrowingRoom),
        KeyCode::Char('s') | KeyCode::Char('2') => Message::SwitchScreen(Screen::Stats),
        KeyCode::Char('a') => Message::ToggleAutoHarvest,
        KeyCode::Char('v') => Message::CycleVisualMode,

        // Harvest key (only works when plant is ready)
        KeyCode::Char('h') => {
            if let Some(ref plant) = app.current_plant {
                if plant.stage == crate::domain::GrowthStage::ReadyToHarvest {
                    return Message::HarvestPlant;
                }
            }
            Message::Tick // No-op if not ready
        },

        _ => Message::Tick, // Ignore other keys
    }
}
