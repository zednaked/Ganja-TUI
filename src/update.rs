use chrono::Utc;

use crate::app::App;
use crate::message::Message;

/// Update function - pure state transformation (The Elm Architecture)
/// Takes current state + message, returns new state
pub fn update(mut app: App, message: Message) -> App {
    match message {
        Message::Tick => {
            // Calculate elapsed time since last tick
            let now = Utc::now();
            let elapsed = now.signed_duration_since(app.last_tick);
            let elapsed_seconds = elapsed.num_milliseconds() as f32 / 1000.0;

            // Update time-based state
            if elapsed_seconds > 0.0 {
                app.update_time(elapsed_seconds);
            }
        }

        Message::SwitchScreen(screen) => {
            app.current_screen = screen;
        }

        Message::Quit => {
            app.running = false;
        }

        Message::HarvestPlant => {
            // Harvest and automatically replant
            app.harvest_and_replant();
        }

        Message::ToggleAutoHarvest => {
            // Toggle full auto mode
            app.toggle_auto_harvest();
        }

        Message::CycleVisualMode => {
            // Cycle to next visual mode
            app.cycle_visual_mode();
        }
    }

    app
}
