use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{Plant, HarvestResult};
use crate::message::Screen;
use crate::ui::colors::{ColorPalette, create_palette};
use crate::ui::visual_mode::VisualMode;

/// Default color palette for deserialization (fallback to Basic16)
fn default_color_palette() -> Box<dyn ColorPalette> {
    create_palette(false, VisualMode::Normal)
}

/// Default visual mode for deserialization
fn default_visual_mode() -> VisualMode {
    VisualMode::Normal
}

/// Main application state (Model in TEA)
#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub current_plant: Option<Plant>,
    pub harvest_history: Vec<HarvestResult>,
    pub last_tick: DateTime<Utc>,
    pub total_harvests: u32,
    pub auto_harvest: bool, // Full auto mode - auto-harvest 10 days after ReadyToHarvest
    #[serde(default = "default_visual_mode")]
    pub visual_mode: VisualMode,

    // UI state (not serialized in some cases, but we'll keep it simple)
    #[serde(skip)]
    pub current_screen: Screen,
    #[serde(skip)]
    pub running: bool,
    #[serde(skip)]
    pub animation_frame: usize,
    #[serde(skip, default = "default_color_palette")]
    pub color_palette: Box<dyn ColorPalette>,
}

impl App {
    /// Create a new application with default state - starts with a plant
    pub fn new(supports_truecolor: bool) -> Self {
        let mut app = Self {
            current_plant: None,
            harvest_history: Vec::new(),
            last_tick: Utc::now(),
            total_harvests: 0,
            auto_harvest: false, // Full auto mode off by default
            visual_mode: VisualMode::Normal,
            current_screen: Screen::GrowingRoom,
            running: true,
            animation_frame: 0,
            color_palette: create_palette(supports_truecolor, VisualMode::Normal),
        };
        // Auto-plant first seed
        app.plant_new_seed();
        app
    }

    /// Plant a new seed with random genetics
    pub fn plant_new_seed(&mut self) {
        self.current_plant = Some(Plant::new_random());
    }

    /// Harvest current plant and auto-plant a new one
    pub fn harvest_and_replant(&mut self) {
        if let Some(plant) = self.current_plant.take() {
            // Calculate harvest result with yield and quality
            let harvest_result = HarvestResult::from_plant(&plant);

            // Record harvest
            self.harvest_history.push(harvest_result);
            self.total_harvests += 1;

            // Auto-plant new seed
            self.plant_new_seed();
        }
    }

    /// Toggle auto-harvest mode on/off
    pub fn toggle_auto_harvest(&mut self) {
        self.auto_harvest = !self.auto_harvest;
    }

    /// Cycle to the next visual mode
    pub fn cycle_visual_mode(&mut self) {
        // Only allow mode cycling in truecolor terminals
        if !self.color_palette.supports_rgb() {
            // In 16-color mode, visual modes don't work well - stay in Normal
            return;
        }

        self.visual_mode = self.visual_mode.next();
        let supports_rgb = self.color_palette.supports_rgb();
        self.color_palette = create_palette(supports_rgb, self.visual_mode);
    }

    /// Update plant state based on elapsed time
    pub fn update_time(&mut self, elapsed_seconds: f32) {
        if let Some(ref mut plant) = self.current_plant {
            // Calculate hours elapsed (50000x speed - ultra fast!)
            // Full cycle (90 days) takes ~6.5 seconds real time
            let hours_elapsed = (elapsed_seconds / 3600.0) * 130000.0;

            // Update total hours elapsed (accelerated time)
            plant.total_hours_elapsed += hours_elapsed;

            // Update days alive based on game hours
            plant.days_alive = (plant.total_hours_elapsed / 24.0) as u32;

            // Update resource consumption based on growth stage (reduced for auto-viewing)
            use crate::domain::GrowthStage;
            let water_drain = match plant.stage {
                GrowthStage::Vegetative => 1.0,
                GrowthStage::Flowering => 0.8,
                _ => 0.5,
            };
            plant.water_level = (plant.water_level - water_drain * hours_elapsed).max(0.0);

            let nutrient_drain = match plant.stage {
                GrowthStage::Vegetative => 0.8,
                GrowthStage::Flowering => 1.0,
                _ => 0.4,
            };
            plant.nutrient_level = (plant.nutrient_level - nutrient_drain * hours_elapsed).max(0.0);

            // Auto-care: keep resources topped up (like watching a bonsai grow)
            if plant.water_level < 40.0 {
                plant.water_level = (plant.water_level + 50.0).min(100.0);
            }
            if plant.nutrient_level < 50.0 {
                plant.nutrient_level = (plant.nutrient_level + 40.0).min(100.0);
            }

            // Update environmental metrics
            // CO2 absorption increases with canopy density
            plant.co2_level = (80.0 + (plant.canopy_density * 0.2)).min(100.0);

            // Light absorption increases with plant size and health
            let light_base = match plant.stage {
                GrowthStage::Seed | GrowthStage::Germination | GrowthStage::Seedling => 40.0,
                GrowthStage::Vegetative => 60.0,
                GrowthStage::PreFlower => 75.0,
                GrowthStage::Flowering | GrowthStage::ReadyToHarvest => 85.0,
            };
            plant.light_absorption = (light_base + (plant.canopy_density * 0.1)).min(100.0);

            // Temperature fluctuates slightly (simulate environment)
            let temp_variation = (plant.days_alive as f32 * 0.7).sin() * 2.0;
            plant.temperature = (24.0 + temp_variation).max(20.0).min(28.0);

            // Humidity affected by watering
            plant.humidity = (50.0 + (plant.water_level * 0.2)).min(80.0);

            // Root development grows over time
            let root_progress = (plant.days_alive as f32 / 90.0 * 100.0).min(100.0);
            plant.root_development = root_progress;

            // Canopy density increases with stage, genetics, and health
            let canopy_base = match plant.stage {
                GrowthStage::Seed | GrowthStage::Germination => 5.0,
                GrowthStage::Seedling => {
                    let base = 15.0;
                    base * plant.genetics.growth_rate
                }
                GrowthStage::Vegetative => {
                    let base = 40.0 + (plant.days_alive as f32 * 0.8);
                    base * plant.genetics.growth_rate
                }
                GrowthStage::PreFlower => {
                    let base = 60.0 + (plant.days_alive as f32 * 0.6);
                    base * plant.genetics.growth_rate
                }
                GrowthStage::Flowering | GrowthStage::ReadyToHarvest => {
                    let base = 80.0 + (plant.days_alive as f32 * 0.2);
                    base * plant.genetics.growth_rate
                }
            };
            plant.canopy_density = canopy_base.min(100.0);

            // Update growth stage
            plant.stage = Plant::calculate_stage(plant.days_alive);

            // Auto-switch to flowering at day 45 if still in veg cycle
            if plant.days_alive >= 45 && plant.light_cycle == crate::domain::LightCycle::Veg18_6 {
                plant.toggle_light_cycle();
            }

            // Update health
            plant.health = Plant::calculate_health(plant.water_level, plant.nutrient_level);

            // Resilience mitiga impacto de health ruim no crescimento
            let health_multiplier = match plant.health {
                crate::domain::HealthStatus::Excellent => 1.0,
                crate::domain::HealthStatus::Good => 1.0,
                crate::domain::HealthStatus::Fair => 0.85 + (plant.genetics.resilience * 0.15),  // 0.85-1.0
                crate::domain::HealthStatus::Poor => 0.65 + (plant.genetics.resilience * 0.35),  // 0.65-1.0
                crate::domain::HealthStatus::Critical => 0.4 + (plant.genetics.resilience * 0.6), // 0.4-1.0
            };

            // Aplicar multiplicador ao canopy_density
            plant.canopy_density *= health_multiplier;

            // Update care history tracking (cumulative)
            let water_optimal = (40.0..=80.0).contains(&plant.water_level);
            let nutrient_optimal = (50.0..=80.0).contains(&plant.nutrient_level);

            if water_optimal {
                plant.care_history.total_optimal_water_hours += hours_elapsed;
            }
            if nutrient_optimal {
                plant.care_history.total_optimal_nutrient_hours += hours_elapsed;
            }
            plant.care_history.total_hours += hours_elapsed;

            // Detect and record stress events
            use crate::domain::{StressEvent, StressSeverity, StressCause};

            if plant.water_level < 20.0 && !plant.care_history.has_recent_stress(StressCause::LowWater, plant.days_alive) {
                plant.care_history.stress_events.push(StressEvent {
                    day: plant.days_alive,
                    severity: StressSeverity::Moderate,
                    cause: StressCause::LowWater,
                });
            }

            if plant.water_level > 90.0 && !plant.care_history.has_recent_stress(StressCause::HighWater, plant.days_alive) {
                plant.care_history.stress_events.push(StressEvent {
                    day: plant.days_alive,
                    severity: StressSeverity::Moderate,
                    cause: StressCause::HighWater,
                });
            }

            if plant.nutrient_level < 30.0 && !plant.care_history.has_recent_stress(StressCause::LowNutrients, plant.days_alive) {
                plant.care_history.stress_events.push(StressEvent {
                    day: plant.days_alive,
                    severity: StressSeverity::Moderate,
                    cause: StressCause::LowNutrients,
                });
            }

            if plant.nutrient_level > 90.0 && !plant.care_history.has_recent_stress(StressCause::NutrientBurn, plant.days_alive) {
                plant.care_history.stress_events.push(StressEvent {
                    day: plant.days_alive,
                    severity: StressSeverity::Severe,
                    cause: StressCause::NutrientBurn,
                });
            }

            // Auto-harvest mode: harvest 10 days after ReadyToHarvest (day 96)
            if self.auto_harvest
                && plant.stage == crate::domain::GrowthStage::ReadyToHarvest
                && plant.days_alive >= 96 {
                // Trigger auto-harvest
                self.harvest_and_replant();
            }
        }

        self.last_tick = Utc::now();
        self.animation_frame = self.animation_frame.wrapping_add(1);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(false) // Default to Basic16 palette
    }
}

impl Clone for App {
    fn clone(&self) -> Self {
        Self {
            current_plant: self.current_plant.clone(),
            harvest_history: self.harvest_history.clone(),
            last_tick: self.last_tick,
            total_harvests: self.total_harvests,
            auto_harvest: self.auto_harvest,
            visual_mode: self.visual_mode,
            current_screen: self.current_screen,
            running: self.running,
            animation_frame: self.animation_frame,
            // Create new palette instance with same visual mode
            color_palette: if self.color_palette.supports_rgb() {
                create_palette(true, self.visual_mode)
            } else {
                create_palette(false, self.visual_mode)
            },
        }
    }
}
