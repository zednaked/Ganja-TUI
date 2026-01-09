use rand::Rng;
use serde::{Deserialize, Serialize};

/// Strain information from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrainInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub strain_type: String,
    pub genetics: String,
    pub thc_min: f32,
    pub thc_max: f32,
    pub cbd_min: f32,
    pub cbd_max: f32,
    pub flowering_time: u32,
    pub difficulty: String,
    pub yield_potential: String,
    pub dominant_terpenes: Vec<String>,
    pub aroma: Vec<String>,
    pub effects: Vec<String>,
    pub height: String,
    pub phenotype: String,
}

/// Genetic traits that determine plant characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genetics {
    /// Base yield potential in grams (50-150g)
    pub yield_potential: f32,
    /// Growth rate multiplier (0.9-1.1)
    pub growth_rate: f32,
    /// Tolerance to care mistakes (0.0-1.0)
    pub resilience: f32,
    /// Maximum achievable quality (70-100%)
    pub quality_ceiling: f32,
    /// Strain information
    pub strain_info: Option<StrainInfo>,
    /// Actual THC % (within strain range)
    pub thc_percent: f32,
    /// Actual CBD % (within strain range)
    pub cbd_percent: f32,
}

impl Genetics {
    /// Load strains from JSON file
    pub fn load_strains() -> Vec<StrainInfo> {
        // Try to load from current directory first, then from installed location
        let paths = [
            "strains.json",
            "./strains.json",
            "/home/zed/ganjatui/strains.json",
        ];

        for path in &paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(strains) = serde_json::from_str::<Vec<StrainInfo>>(&content) {
                    return strains;
                }
            }
        }

        // Fallback to empty vec if file not found
        Vec::new()
    }

    /// Generate random genetics for a new seed with strain data
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let strains = Self::load_strains();

        let strain_info = if !strains.is_empty() {
            Some(strains[rng.gen_range(0..strains.len())].clone())
        } else {
            None
        };

        // Generate genetics based on strain or random
        let (yield_potential, resilience, quality_ceiling, thc_percent, cbd_percent) = if let Some(ref strain) = strain_info {
            let yield_base = match strain.yield_potential.as_str() {
                "High" => rng.gen_range(100.0..=150.0),
                "Medium" => rng.gen_range(70.0..=110.0),
                "Low" => rng.gen_range(50.0..=80.0),
                _ => rng.gen_range(50.0..=150.0),
            };

            let resilience_val = match strain.difficulty.as_str() {
                "Easy" => rng.gen_range(0.7..=1.0),
                "Medium" => rng.gen_range(0.4..=0.7),
                "Hard" => rng.gen_range(0.0..=0.4),
                _ => rng.gen_range(0.0..=1.0),
            };

            let quality_val = match strain.strain_type.as_str() {
                "Sativa" | "Indica" => rng.gen_range(80.0..=100.0),
                "Hybrid" => rng.gen_range(85.0..=100.0),
                _ => rng.gen_range(70.0..=100.0),
            };

            let thc = rng.gen_range(strain.thc_min..=strain.thc_max);
            let cbd = rng.gen_range(strain.cbd_min..=strain.cbd_max);

            (yield_base, resilience_val, quality_val, thc, cbd)
        } else {
            // Random genetics if no strain data
            (
                rng.gen_range(50.0..=150.0),
                rng.gen_range(0.0..=1.0),
                rng.gen_range(70.0..=100.0),
                rng.gen_range(15.0..=25.0),
                rng.gen_range(0.1..=1.0),
            )
        };

        Self {
            yield_potential,
            growth_rate: rng.gen_range(0.9..=1.1),
            resilience,
            quality_ceiling,
            strain_info,
            thc_percent,
            cbd_percent,
        }
    }
}
