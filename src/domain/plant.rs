use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::genetics::Genetics;

/// Growth stages of the plant
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrowthStage {
    Seed,
    Germination,    // Days 1-3
    Seedling,       // Days 4-14
    Vegetative,     // Days 15-45
    PreFlower,      // Days 46-52
    Flowering,      // Days 53-90
    ReadyToHarvest, // Days 90+
}

impl GrowthStage {
    /// Get the stage name as a string
    pub fn as_str(&self) -> &str {
        match self {
            GrowthStage::Seed => "Seed",
            GrowthStage::Germination => "Germination",
            GrowthStage::Seedling => "Seedling",
            GrowthStage::Vegetative => "Vegetative",
            GrowthStage::PreFlower => "Pre-Flower",
            GrowthStage::Flowering => "Flowering",
            GrowthStage::ReadyToHarvest => "Ready to Harvest",
        }
    }
}

/// Light cycle settings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LightCycle {
    /// 18 hours on, 6 hours off (vegetative)
    Veg18_6,
    /// 12 hours on, 12 hours off (flowering)
    Flower12_12,
}

/// Plant health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

/// Stress event severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressSeverity {
    Minor,
    Moderate,
    Severe,
}

/// Cause of stress
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StressCause {
    LowWater,
    HighWater,
    LowNutrients,
    NutrientBurn,
    WrongLightCycle,
}

/// A stress event recorded in care history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressEvent {
    pub day: u32,
    pub severity: StressSeverity,
    pub cause: StressCause,
}

/// History of care quality for quality calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CareHistory {
    /// Total game hours elapsed (for percentage calculations)
    #[serde(default)]
    pub total_hours: f32,
    /// Total hours water was in optimal range (40-80%)
    #[serde(default)]
    pub total_optimal_water_hours: f32,
    /// Total hours nutrients were in optimal range (50-80%)
    #[serde(default)]
    pub total_optimal_nutrient_hours: f32,

    /// Deprecated - use calculate_water_percentage() instead
    #[serde(default = "default_percentage")]
    pub water_optimal_percentage: f32,
    /// Deprecated - use calculate_nutrient_percentage() instead
    #[serde(default = "default_percentage")]
    pub nutrient_optimal_percentage: f32,

    /// Whether light cycle was appropriate for stages
    pub light_cycle_correct: bool,
    /// Recorded stress events
    pub stress_events: Vec<StressEvent>,
}

fn default_percentage() -> f32 {
    100.0
}

impl CareHistory {
    /// Calculate actual water percentage based on cumulative tracking
    pub fn calculate_water_percentage(&self) -> f32 {
        if self.total_hours == 0.0 {
            100.0
        } else {
            (self.total_optimal_water_hours / self.total_hours) * 100.0
        }
    }

    /// Calculate actual nutrient percentage based on cumulative tracking
    pub fn calculate_nutrient_percentage(&self) -> f32 {
        if self.total_hours == 0.0 {
            100.0
        } else {
            (self.total_optimal_nutrient_hours / self.total_hours) * 100.0
        }
    }

    /// Check if a recent stress event of this cause was already recorded
    /// Prevents spam of events - only records if no event of same cause in last 5 days
    pub fn has_recent_stress(&self, cause: StressCause, current_day: u32) -> bool {
        self.stress_events
            .iter()
            .rev()
            .take(10)
            .any(|e| e.cause == cause && e.day >= current_day.saturating_sub(5))
    }
}

impl Default for CareHistory {
    fn default() -> Self {
        Self {
            total_hours: 0.0,
            total_optimal_water_hours: 0.0,
            total_optimal_nutrient_hours: 0.0,
            water_optimal_percentage: 100.0,
            nutrient_optimal_percentage: 100.0,
            light_cycle_correct: true,
            stress_events: Vec::new(),
        }
    }
}

/// The main plant structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plant {
    pub id: Uuid,
    pub strain_name: String,
    pub stage: GrowthStage,
    pub planted_at: DateTime<Utc>,
    pub days_alive: u32,
    pub total_hours_elapsed: f32, // Track game time (accelerated)
    pub water_level: f32,     // 0-100%
    pub nutrient_level: f32,  // 0-100%
    pub light_cycle: LightCycle,
    pub health: HealthStatus,
    pub genetics: Genetics,
    pub care_history: CareHistory,

    // Environmental metrics
    pub co2_level: f32,           // 0-100% (CO2 absorption/availability)
    pub light_absorption: f32,    // 0-100% (photosynthesis efficiency)
    pub temperature: f32,         // Celsius (20-28°C optimal)
    pub humidity: f32,            // 0-100% (50-70% optimal)
    pub root_development: f32,    // 0-100% (root system strength)
    pub canopy_density: f32,      // 0-100% (foliage coverage)
}

impl Plant {
    /// Create a new plant with random genetics
    pub fn new_random() -> Self {
        let genetics = Genetics::random();
        let strain_name = genetics.strain_info
            .as_ref()
            .map(|s| s.name.clone())
            .unwrap_or_else(|| "Unknown Strain".to_string());

        Self {
            id: Uuid::new_v4(),
            strain_name,
            stage: GrowthStage::Seedling,  // Start directly as seedling
            planted_at: Utc::now(),
            days_alive: 1,  // Start at day 1
            total_hours_elapsed: 0.0,
            water_level: 60.0,
            nutrient_level: 60.0,
            light_cycle: LightCycle::Veg18_6,
            health: HealthStatus::Excellent,
            genetics,
            care_history: CareHistory::default(),
            co2_level: 80.0,
            light_absorption: 50.0,
            temperature: 24.0,
            humidity: 60.0,
            root_development: 10.0,
            canopy_density: 5.0,
        }
    }

    // Removed new() method - use new_random() instead

    /// Calculate growth stage based on days alive
    pub fn calculate_stage(days: u32) -> GrowthStage {
        match days {
            1..=10 => GrowthStage::Seedling,      // Days 1-10: small seedling
            11..=40 => GrowthStage::Vegetative,   // Days 11-40: vegetative growth
            41..=48 => GrowthStage::PreFlower,    // Days 41-48: pre-flower
            49..=85 => GrowthStage::Flowering,    // Days 49-85: flowering
            _ => GrowthStage::ReadyToHarvest,     // Days 86+: ready to harvest
        }
    }

    /// Calculate health based on current resource levels
    pub fn calculate_health(water: f32, nutrients: f32) -> HealthStatus {
        let water_optimal = water >= 40.0 && water <= 80.0;
        let nutrient_optimal = nutrients >= 50.0 && nutrients <= 80.0;

        let water_critical = water < 10.0 || water > 95.0;
        let nutrient_critical = nutrients < 20.0 || nutrients > 95.0;

        if water_critical || nutrient_critical {
            HealthStatus::Critical
        } else if !water_optimal && !nutrient_optimal {
            HealthStatus::Poor
        } else if !water_optimal || !nutrient_optimal {
            HealthStatus::Fair
        } else if water >= 50.0 && water <= 70.0 && nutrients >= 60.0 && nutrients <= 75.0 {
            HealthStatus::Excellent
        } else {
            HealthStatus::Good
        }
    }

    // Removed water() and feed() methods - plant is auto-managed now

    /// Toggle light cycle
    pub fn toggle_light_cycle(&mut self) {
        self.light_cycle = match self.light_cycle {
            LightCycle::Veg18_6 => LightCycle::Flower12_12,
            LightCycle::Flower12_12 => LightCycle::Veg18_6,
        };
    }
}
