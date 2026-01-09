use crate::domain::GrowthStage;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref PLANT_CACHE: Mutex<HashMap<u64, PlantStructure>> = Mutex::new(HashMap::new());
}

/// Phenotype determines growth pattern
#[derive(Clone, Copy, Debug)]
pub enum Phenotype {
    Tall,       // Sativa-like: tall, thin branches, spaced out
    Bushy,      // Indica-like: short, dense, many branches
    Balanced,   // Hybrid: balanced growth
}

/// Plant structure - procedurally generated for each plant
#[derive(Clone, Debug)]
pub struct PlantStructure {
    pub branches: Vec<Branch>,
    #[allow(dead_code)]
    pub seed: u64,
    #[allow(dead_code)]
    pub phenotype: Phenotype,
    #[allow(dead_code)]
    pub branch_density: f32,
    pub foliage_density: f32,
    pub trunk_splits: Vec<TrunkSplit>, // Main trunk bifurcations
    pub max_height: usize,              // Maximum height this plant can reach
    pub growth_rate: f32,               // How fast trunk grows (per day)
}

#[derive(Clone, Debug)]
pub struct TrunkSplit {
    pub split_day: u32,     // Day when trunk splits
    pub split_level: usize, // Height where split occurs
    pub angle: i8,          // Direction offset (-2 to 2)
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub level: usize,           // Height level (0-12)
    pub direction: i8,          // -1 = left, 1 = right
    pub growth_start_day: u32,  // Day this branch starts growing
    pub max_length: u8,         // Maximum length this branch can reach
    pub thickness: u8,          // Branch thickness (1-3)
    #[allow(dead_code)]
    pub is_secondary: bool,     // Secondary branch (grows from another branch)
    #[allow(dead_code)]
    pub parent_index: Option<usize>, // Index of parent branch if secondary
    pub curve: i8,              // Branch curvature (-1, 0, 1)
    pub can_bifurcate: bool,    // Can this branch split into 2?
    pub bifurcation_day: u32,   // Day when branch bifurcates (if can_bifurcate)
}

impl PlantStructure {
    /// Get or generate a cached plant structure
    pub fn get_or_generate(seed: u64) -> Self {
        let mut cache = PLANT_CACHE.lock().unwrap();

        if let Some(structure) = cache.get(&seed) {
            return structure.clone();
        }

        let structure = Self::generate(seed);
        cache.insert(seed, structure.clone());
        structure
    }

    /// Generate a unique plant structure based on seed
    fn generate(seed: u64) -> Self {
        let mut rng = SimpleRng::new(seed);

        // Determine phenotype
        let phenotype = match rng.next() % 3 {
            0 => Phenotype::Tall,
            1 => Phenotype::Bushy,
            _ => Phenotype::Balanced,
        };

        let (branch_density, foliage_density, max_height, growth_rate) = match phenotype {
            Phenotype::Tall => (0.6, 0.4, 20 + (rng.next() % 5) as usize, 0.25),      // 20-24 height, reaches max ~96 days
            Phenotype::Bushy => (1.0, 0.9, 12 + (rng.next() % 5) as usize, 0.22),     // 12-16 height, reaches max ~64 days
            Phenotype::Balanced => (0.8, 0.7, 16 + (rng.next() % 5) as usize, 0.23),  // 16-20 height, reaches max ~80 days
        };

        // MANY more primary branches - they appear early and frequently
        let num_primary = match phenotype {
            Phenotype::Tall => 15 + (rng.next() % 10) as usize,      // 15-25 primary
            Phenotype::Bushy => 25 + (rng.next() % 15) as usize,     // 25-40 primary
            Phenotype::Balanced => 20 + (rng.next() % 12) as usize,  // 20-32 primary
        };

        let mut branches = Vec::new();

        // Generate PRIMARY branches (from trunk)
        // Branches appear as trunk grows to their height
        for _i in 0..num_primary {
            // Level distribution - spread across the plant height
            let level = match phenotype {
                Phenotype::Tall => 1 + (rng.next() % (max_height - 1) as u64) as usize,      // 1 to max_height
                Phenotype::Bushy => 2 + (rng.next() % (max_height - 2).max(1) as u64) as usize, // Lower/middle
                Phenotype::Balanced => 2 + (rng.next() % (max_height - 2).max(1) as u64) as usize,
            };

            // Branches appear MUCH earlier - starting when trunk is only 4 levels tall
            // Formula adjusted so branches appear quickly after trunk reaches their level
            let days_per_level = match phenotype {
                Phenotype::Tall => 1.2,   // Very fast branch appearance
                Phenotype::Bushy => 0.8,  // Even faster
                Phenotype::Balanced => 1.0,
            };

            // Branches can start appearing from day 4 onwards (when trunk_height = 4)
            let base_day = 4;
            let level_day = base_day + ((max_height - level) as f32 * days_per_level) as u32;
            let growth_start_day = level_day + (rng.next() % 3) as u32; // Small variation

            let direction = if rng.next() % 2 == 0 { -1 } else { 1 };

            // Longer branches - doubled size for 2x bigger plants
            let max_length = match phenotype {
                Phenotype::Tall => 6 + (rng.next() % 8) as u8,         // 6-13 chars
                Phenotype::Bushy => 8 + (rng.next() % 6) as u8,        // 8-13 chars
                Phenotype::Balanced => 6 + (rng.next() % 8) as u8,     // 6-13 chars
            };

            let thickness = match phenotype {
                Phenotype::Tall => 1,
                Phenotype::Bushy => if rng.next() % 2 == 0 { 2 } else { 1 },
                Phenotype::Balanced => if rng.next() % 3 == 0 { 2 } else { 1 },
            };

            // Add some curvature
            let curve = if rng.next() % 3 == 0 {
                if rng.next() % 2 == 0 { -1 } else { 1 }
            } else {
                0
            };

            // More branches can bifurcate (split into 2)
            let can_bifurcate = rng.next() % 3 == 0; // 33% chance (increased from 20%)
            let bifurcation_day = if can_bifurcate {
                growth_start_day + 8 + (rng.next() % 8) as u32 // Bifurcate sooner
            } else {
                999 // Never
            };

            branches.push(Branch {
                level,
                direction,
                growth_start_day,
                max_length,
                thickness,
                is_secondary: false,
                parent_index: None,
                curve,
                can_bifurcate,
                bifurcation_day,
            });
        }

        // Generate SECONDARY branches (from primary branches) - MORE of them
        let num_secondary = match phenotype {
            Phenotype::Tall => (num_primary as f32 * 0.5) as usize,      // 50% secondaries
            Phenotype::Bushy => (num_primary as f32 * 0.8) as usize,     // 80% secondaries
            Phenotype::Balanced => (num_primary as f32 * 0.6) as usize,  // 60% secondaries
        };

        let primary_count = branches.len();
        for _ in 0..num_secondary {
            let parent_idx = (rng.next() as usize) % primary_count;
            let parent = &branches[parent_idx];

            // Secondary branches appear sooner
            let growth_start_day = parent.growth_start_day + 5 + (rng.next() % 5) as u32;

            // Same level or one level up/down from parent
            let level_offset = ((rng.next() % 3) as i32) - 1; // -1, 0, or 1
            let level = ((parent.level as i32 + level_offset).max(1).min((max_height - 1) as i32)) as usize;

            // Often grows opposite direction for visual variety
            let direction = if rng.next() % 3 == 0 {
                parent.direction
            } else {
                -parent.direction
            };

            // Longer secondary branches for 2x size
            let max_length = 4 + (rng.next() % 6) as u8; // 4-9 chars

            let thickness = 1; // Thinner

            let curve = if rng.next() % 2 == 0 {
                if rng.next() % 2 == 0 { -1 } else { 1 }
            } else {
                0
            };

            // Secondary branches can also bifurcate more often
            let can_bifurcate = rng.next() % 5 == 0; // 20% chance
            let bifurcation_day = if can_bifurcate {
                growth_start_day + 10 + (rng.next() % 8) as u32
            } else {
                999
            };

            branches.push(Branch {
                level,
                direction,
                growth_start_day,
                max_length,
                thickness,
                is_secondary: true,
                parent_index: Some(parent_idx),
                curve,
                can_bifurcate,
                bifurcation_day,
            });
        }

        // Generate trunk splits (bifurcations)
        let mut trunk_splits = Vec::new();
        let num_splits = match phenotype {
            Phenotype::Tall => if rng.next() % 3 == 0 { 1 } else { 0 },      // 33% chance
            Phenotype::Bushy => if rng.next() % 2 == 0 { 1 } else { 2 },     // Often splits
            Phenotype::Balanced => if rng.next() % 4 == 0 { 1 } else { 0 },  // 25% chance
        };

        for _ in 0..num_splits {
            let split_day = 20 + (rng.next() % 30) as u32; // Day 20-50
            let split_level = 4 + (rng.next() % 4) as usize; // Level 4-8
            let angle = ((rng.next() % 5) as i8) - 2; // -2 to 2

            trunk_splits.push(TrunkSplit {
                split_day,
                split_level,
                angle,
            });
        }

        Self {
            branches,
            seed,
            phenotype,
            branch_density,
            foliage_density,
            trunk_splits,
            max_height,
            growth_rate,
        }
    }

    /// Calculate current trunk height based on day
    pub fn trunk_height(&self, day: u32) -> usize {
        // Trunk grows progressively based on growth_rate
        // Formula: height = min(day * growth_rate, max_height)
        let calculated_height = (day as f32 * self.growth_rate) as usize;
        calculated_height.min(self.max_height)
    }

    /// Calculate current length using sigmoid growth curve
    pub fn branch_length(&self, branch: &Branch, current_day: u32) -> f32 {
        if current_day < branch.growth_start_day {
            return 0.0;
        }

        let days_growing = (current_day - branch.growth_start_day) as f32;
        let total_days = branch.max_length as f32 * 3.0;
        let progress = (days_growing / total_days).min(1.0);

        // Sigmoid growth curve (slow -> fast -> slow)
        let sigmoid = 1.0 / (1.0 + (-8.0 * (progress - 0.5)).exp());

        branch.max_length as f32 * sigmoid
    }

    /// Get branches that are currently visible (started growing)
    pub fn visible_branches(&self, day: u32) -> Vec<&Branch> {
        self.branches.iter()
            .filter(|b| b.growth_start_day <= day)
            .collect()
    }

    /// Calculate foliage density for a specific day
    pub fn current_foliage_density(&self, day: u32) -> f32 {
        // Foliage increases over time
        let max_day = 90.0;
        let progress = (day as f32 / max_day).min(1.0);
        self.foliage_density * progress
    }
}

/// Simple pseudo-random number generator for deterministic plant generation
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed.wrapping_add(1) }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        (self.state / 65536) % 32768
    }
}

/// Get plant ASCII art - procedurally generated and animated
pub fn get_plant_ascii(stage: GrowthStage, day: u32, seed: u64, frame: usize) -> Vec<String> {
    let structure = PlantStructure::get_or_generate(seed);

    match stage {
        // No more Seed or Germination - start directly as Seedling
        GrowthStage::Seed | GrowthStage::Germination => render_seedling(day, &structure, frame, stage),
        GrowthStage::Seedling => render_seedling(day, &structure, frame, stage),
        GrowthStage::Vegetative => render_vegetative(day, &structure, frame, stage),
        GrowthStage::PreFlower => render_preflower(day, &structure, frame, stage),
        GrowthStage::Flowering => render_flowering(day, &structure, frame, stage),
        GrowthStage::ReadyToHarvest => render_harvest(day, &structure, frame, stage),
    }
}

// Removed render_seed() and render_germination() - plants start directly as seedlings

fn render_seedling(day: u32, structure: &PlantStructure, frame: usize, stage: GrowthStage) -> Vec<String> {
    render_plant_structure(day, structure, frame, false, "", stage)
}

fn render_vegetative(day: u32, structure: &PlantStructure, frame: usize, stage: GrowthStage) -> Vec<String> {
    render_plant_structure(day, structure, frame, false, "", stage)
}

fn render_preflower(day: u32, structure: &PlantStructure, frame: usize, stage: GrowthStage) -> Vec<String> {
    // 8-frame gentle appearance of small flowers
    let flowers = ['.', '*', '.', ' ', '.', '*', '.', ' '];
    let flower = &flowers[frame % 8].to_string();
    render_plant_structure(day, structure, frame, true, flower, stage)
}

fn render_flowering(day: u32, structure: &PlantStructure, frame: usize, stage: GrowthStage) -> Vec<String> {
    // 12-frame pulsing/breathing buds
    let buds = ['o', 'o', 'O', 'O', '@', '@', 'O', 'O', 'o', 'o', '.', '.'];
    let bud = &buds[frame % 12].to_string();
    render_plant_structure(day, structure, frame, true, bud, stage)
}

fn render_harvest(day: u32, structure: &PlantStructure, frame: usize, stage: GrowthStage) -> Vec<String> {
    // 8-frame trichome sparkle effect
    let harvest = ['@', '#', '@', '*', '#', '@', '*', '#'];
    let bud = &harvest[frame % 8].to_string();
    render_plant_structure(day, structure, frame, true, bud, stage)
}

/// Render the plant structure into ASCII art
/// ALWAYS returns exactly 70 chars wide × 28 lines tall
fn render_plant_structure(
    day: u32,
    structure: &PlantStructure,
    frame: usize,
    show_flowers: bool,
    flower_char: &str,
    stage: GrowthStage,
) -> Vec<String> {
    // Create 28 lines buffer (70 chars wide) - DOUBLE SIZE
    let mut lines: Vec<Vec<char>> = vec![vec![' '; 70]; 28];

    // Draw main trunk with progressive growth
    // Trunk animation varies by stage
    let trunk_char = match stage {
        GrowthStage::Seed | GrowthStage::Germination | GrowthStage::Seedling => {
            // Seedling: 2-frame fast, energetic
            let chars = ['|', '!'];
            chars[frame % 2]
        }
        GrowthStage::Vegetative => {
            // Vegetative: 3-frame standard
            let chars = ['|', '!', 'I'];
            chars[frame % 3]
        }
        GrowthStage::PreFlower | GrowthStage::Flowering => {
            // Flowering: 4-frame thicker appearance
            let chars = ['|', '!', 'I', '║'];
            chars[frame % 4]
        }
        GrowthStage::ReadyToHarvest => {
            // Harvest: 2-frame stable, mature
            let chars = ['I', '║'];
            chars[frame % 2]
        }
    };

    let center = 35; // Center position (middle of 70)

    // Calculate current trunk height (grows progressively)
    let current_trunk_height = structure.trunk_height(day);

    // Trunk grows from bottom (27) upward
    // Only draw trunk up to current height
    let trunk_start_level = (27 - current_trunk_height).max(0);

    // Check for active splits
    let active_splits: Vec<&TrunkSplit> = structure.trunk_splits.iter()
        .filter(|s| s.split_day <= day)
        .collect();

    let mut split_found = false;
    let mut split_level_found = 0;

    for level in trunk_start_level..=27 {
        let trunk = trunk_char;

        // Check if there's a split at this level
        let split_here = active_splits.iter().find(|s| s.split_level == (27 - level));

        if let Some(split) = split_here {
            if !split_found {
                // Draw bifurcation
                lines[level][center] = trunk;

                // Draw the split branches going outward
                let split_pos_left = (center as i8 - split.angle.abs()) as usize;
                let split_pos_right = (center as i8 + split.angle.abs()) as usize;

                if split_pos_left < 70 && level > 0 {
                    lines[level - 1][split_pos_left] = if split.angle < 0 { '\\' } else { '/' };
                }
                if split_pos_right < 70 && level > 0 {
                    lines[level - 1][split_pos_right] = if split.angle > 0 { '/' } else { '\\' };
                }

                // Continue both branches upward from split point
                if level >= 2 {
                    for up_level in (trunk_start_level..level-1).rev() {
                        if split_pos_left < 70 {
                            lines[up_level][split_pos_left] = trunk;
                        }
                        if split_pos_right < 70 {
                            lines[up_level][split_pos_right] = trunk;
                        }
                    }
                }

                split_found = true;
                split_level_found = level;
            }
        } else if !split_found || level > split_level_found {
            // Draw normal trunk (either no split yet, or below the split point)
            lines[level][center] = trunk;
        }
    }

    // Get visible branches for this day
    let visible = structure.visible_branches(day);

    // Get foliage density
    let foliage_density = structure.current_foliage_density(day);

    // Draw branches growing from trunk outward
    for branch in visible.iter() {
        let level = 27 - branch.level; // Invert level (0 is top, 27 is bottom)
        if level >= 27 { continue; }

        // Only draw branch if trunk has reached its level
        if branch.level > current_trunk_height {
            continue; // Trunk hasn't grown to this branch yet
        }

        let current_length = structure.branch_length(branch, day);
        if current_length < 0.5 { continue; }

        let length_int = current_length.ceil() as u8;

        // Check if branch is bifurcating
        let is_bifurcating = branch.can_bifurcate && day >= branch.bifurcation_day;

        // Draw the branch with curvature
        for i in 1..=length_int {
            let x_pos = center as i8 + (i as i8 * branch.direction);
            let mut y_pos = level as i8;

            // Apply curvature - branch bends up or down
            if branch.curve != 0 && i > 2 {
                let curve_amount = ((i - 2) as i8 / 2) * branch.curve;
                y_pos = (y_pos - curve_amount).max(0).min(27);
            }

            // Skip if out of bounds
            if x_pos < 0 || x_pos >= 70 || y_pos < 0 || y_pos >= 28 { break; }

            let x = x_pos as usize;
            let y = y_pos as usize;

            // Choose character based on position, curve, and density
            let ch = if i == length_int && show_flowers {
                // Flower/bud at the tip
                flower_char.chars().next().unwrap_or('*')
            } else if i == 1 {
                // Near trunk - use connection character
                if branch.direction < 0 { '\\' } else { '/' }
            } else if i == length_int {
                // At tip without flowers
                if foliage_density > 0.6 {
                    if branch.direction < 0 { '\\' } else { '/' }
                } else {
                    if branch.direction < 0 { '/' } else { '\\' }
                }
            } else {
                // Middle of branch - consider curvature
                if branch.curve != 0 && i > 2 {
                    // Curved branch uses different chars
                    if branch.curve > 0 { '/' } else { '\\' }
                } else {
                    // Straight branch
                    match branch.thickness {
                        2 => '=',      // Thick branch
                        3 => '#',      // Very thick
                        _ => '_',      // Normal branch
                    }
                }
            };

            // Only draw if space is empty (don't overwrite)
            if lines[y][x] == ' ' {
                lines[y][x] = ch;
            }
        }

        // Add foliage density effect
        if foliage_density > 0.5 && length_int >= 3 && level > 0 {
            for offset in 1..=2 {
                let foliage_x_pos = center as i8 + ((length_int - offset) as i8 * branch.direction);
                let foliage_y = level - 1;

                if foliage_x_pos > 0 && foliage_x_pos < 34 && foliage_y < 14 {
                    let fx = foliage_x_pos as usize;
                    if lines[foliage_y][fx] == ' ' && foliage_density > 0.6 {
                        lines[foliage_y][fx] = if show_flowers {
                            if offset == 1 { '*' } else { '.' }
                        } else {
                            ':'
                        };
                    }
                }
            }
        }

        // Branch bifurcation - split into 2 sub-branches
        if is_bifurcating && length_int >= 3 {
            // Split point is 2/3 along the branch
            let split_point = (length_int * 2 / 3).max(2);

            // Two sub-branches grow from split point
            for sub_dir in [-1, 1].iter() {
                for i in 1..=2 {
                    let base_x = center as i8 + (split_point as i8 * branch.direction);
                    let x_pos = base_x + (i * sub_dir);
                    let y_pos = level as i8 - (i / 2); // Slightly upward

                    if x_pos >= 0 && x_pos < 70 && y_pos >= 0 && y_pos < 28 {
                        let x = x_pos as usize;
                        let y = y_pos as usize;

                        let ch = if i == 2 && show_flowers {
                            flower_char.chars().next().unwrap_or('*')
                        } else if *sub_dir < 0 { '\\' } else { '/' };

                        if lines[y][x] == ' ' {
                            lines[y][x] = ch;
                        }
                    }
                }
            }
        }
    }

    // Draw soil line (wider, doubled size)
    let soil = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
    for (i, c) in soil.chars().enumerate() {
        let x = 16 + i;
        if x < 70 {
            lines[27][x] = c;
        }
    }

    // Convert to strings - GUARANTEE 70 chars per line
    lines.into_iter()
        .map(|line| {
            let s: String = line.into_iter().collect();
            // Ensure exactly 70 chars
            format!("{:70}", s.get(..70).unwrap_or(&s))
        })
        .collect()
}

// Removed get_drying_ascii() - no longer have drying room feature

/// Get animated border decoration
pub fn get_border_decoration(frame: usize) -> &'static str {
    let decorations = ["~", "~", "-", "-"];
    decorations[frame % decorations.len()]
}

/// Get animated water drops
pub fn get_water_drops(frame: usize) -> &'static str {
    let drops = [".", "o", ".", "O", ".", "o", ".", " "];
    drops[frame % drops.len()]
}

/// Get animated nutrient sparkles
pub fn get_nutrient_sparkles(frame: usize) -> &'static str {
    let sparkles = ["*", "+", "*", "x", "*", "+", "*", "X", "*", "x", "*", " "];
    sparkles[frame % sparkles.len()]
}

// Removed get_jar_ascii() and get_fill() - no longer have jar/curing feature
