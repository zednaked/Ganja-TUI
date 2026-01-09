/// All possible messages/events in the application
/// Following The Elm Architecture pattern
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Quit,
    HarvestPlant,
    ToggleAutoHarvest,
    CycleVisualMode,
    SwitchScreen(Screen),
}

/// Screen selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    GrowingRoom,
    Stats,
}
