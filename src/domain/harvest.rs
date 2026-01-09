use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::plant::Plant;

/// Result of harvesting a plant with calculated yield and quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestResult {
    pub strain_name: String,
    pub harvest_day: u32,
    pub completed_at: DateTime<Utc>,
    pub weight_grams: f32,
    pub quality_score: f32,  // 0-100
    pub thc_percent: f32,
    pub cbd_percent: f32,
}

impl HarvestResult {
    /// Calculate harvest result from a plant
    pub fn from_plant(plant: &Plant) -> Self {
        // Base yield from genetics (50-150g range)
        let base_yield = plant.genetics.yield_potential;

        // Care quality multiplier based on optimal conditions (0.7-1.0)
        let water_pct = plant.care_history.calculate_water_percentage();
        let nutrient_pct = plant.care_history.calculate_nutrient_percentage();
        let care_quality = ((water_pct + nutrient_pct) / 200.0).max(0.7);

        // Stress penalty - each stress event reduces yield by 2% (max -30%)
        let stress_count = plant.care_history.stress_events.len();
        let stress_penalty = (stress_count as f32 * 0.02).min(0.3);

        // Final weight calculation
        let weight_grams = base_yield * care_quality * (1.0 - stress_penalty);

        // Quality score (0-100) based on care and stress
        let quality_score = (care_quality * 100.0 * (1.0 - stress_penalty))
            .clamp(0.0, 100.0);

        // Cannabinoid content affected by quality (0.7-1.0 multiplier)
        let cannabinoid_multiplier = 0.7 + (quality_score / 100.0 * 0.3);
        let thc_percent = plant.genetics.thc_percent * cannabinoid_multiplier;
        let cbd_percent = plant.genetics.cbd_percent * cannabinoid_multiplier;

        HarvestResult {
            strain_name: plant.strain_name.clone(),
            harvest_day: plant.days_alive,
            completed_at: Utc::now(),
            weight_grams,
            quality_score,
            thc_percent,
            cbd_percent,
        }
    }
}
