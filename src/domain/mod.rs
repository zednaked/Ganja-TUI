pub mod genetics;
pub mod harvest;
pub mod plant;

pub use harvest::HarvestResult;
pub use plant::{
    GrowthStage, HealthStatus, LightCycle, Plant,
    StressEvent, StressSeverity, StressCause,
};
