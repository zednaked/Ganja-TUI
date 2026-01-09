use ratatui::style::Color;
use crate::domain::GrowthStage;
use std::fmt::Debug;

/// Flower color intensity based on growth stage
#[derive(Debug, Clone, Copy)]
pub enum FlowerIntensity {
    Early,      // Pre-flower stage
    Developing, // Early flowering
    Peak,       // Late flowering
    Harvest,    // Ready to harvest - maximum vibrance
}

/// Color palette trait for terminal color support detection
pub trait ColorPalette: Debug + Send + Sync {
    /// Get flower color based on genetic variant, intensity, and stage
    fn flower_color(&self, variant: u8, intensity: FlowerIntensity, stage: GrowthStage) -> Color;

    /// Get foliage color based on variant, health, and water level
    fn foliage_color(&self, variant: u8, health: f32, water: f32) -> Color;

    /// Get trunk color based on variant and plant age
    fn trunk_color(&self, variant: u8, age_days: u32) -> Color;

    /// Get soil color based on moisture level
    fn soil_color(&self, moisture: f32) -> Color;

    /// Get water gauge color based on water level (0-100)
    fn water_color(&self, level: f32) -> Color;

    /// Get nutrient gauge color based on nutrient level (0-100)
    fn nutrient_color(&self, level: f32) -> Color;

    /// Get background tint for current stage (returns None if not supported)
    fn background_tint(&self, stage: GrowthStage) -> Option<Color>;

    /// Check if palette supports RGB colors
    fn supports_rgb(&self) -> bool;
}

/// Basic 16-color ANSI palette (fallback, current system)
#[derive(Debug)]
pub struct Basic16Palette;

impl Basic16Palette {
    pub fn new() -> Self {
        Basic16Palette
    }
}

impl ColorPalette for Basic16Palette {
    fn flower_color(&self, variant: u8, intensity: FlowerIntensity, _stage: GrowthStage) -> Color {
        // Current 6-variant flower color system
        let base_color = match variant % 6 {
            0 => Color::Magenta,      // Purple
            1 => Color::Red,          // Orange/Red
            2 => Color::Yellow,       // Golden
            3 => Color::LightMagenta, // Pink
            4 => Color::Cyan,         // Blue/Teal
            5 => Color::White,        // Frosty
            _ => Color::Magenta,
        };

        // Adjust for intensity (limited in 16-color mode)
        match intensity {
            FlowerIntensity::Early => match base_color {
                Color::Magenta => Color::Magenta,
                Color::Red => Color::Red,
                Color::Yellow => Color::Yellow,
                Color::LightMagenta => Color::LightMagenta,
                Color::Cyan => Color::Cyan,
                _ => Color::White,
            },
            FlowerIntensity::Developing => match base_color {
                Color::Magenta => Color::LightMagenta,
                Color::Red => Color::LightRed,
                Color::Yellow => Color::LightYellow,
                Color::LightMagenta => Color::Magenta,
                Color::Cyan => Color::LightCyan,
                _ => Color::White,
            },
            FlowerIntensity::Peak | FlowerIntensity::Harvest => match base_color {
                Color::Magenta => Color::LightMagenta,
                Color::Red => Color::LightRed,
                Color::Yellow => Color::LightYellow,
                Color::LightMagenta => Color::LightMagenta,
                Color::Cyan => Color::LightCyan,
                _ => Color::White,
            },
        }
    }

    fn foliage_color(&self, variant: u8, _health: f32, _water: f32) -> Color {
        // Current 4-variant foliage system, ignore environmental factors in 16-color mode
        match variant % 4 {
            0 | 2 => Color::Green,
            1 | 3 => Color::LightGreen,
            _ => Color::Green,
        }
    }

    fn trunk_color(&self, variant: u8, _age_days: u32) -> Color {
        // Current 3-variant trunk system, ignore age in 16-color mode
        match variant % 3 {
            0 => Color::Yellow,    // Light brown
            1 => Color::Red,       // Reddish brown
            2 => Color::DarkGray,  // Dark brown
            _ => Color::Yellow,
        }
    }

    fn soil_color(&self, _moisture: f32) -> Color {
        // Fixed yellow soil, ignore moisture in 16-color mode
        Color::Yellow
    }

    fn water_color(&self, level: f32) -> Color {
        // Basic threshold-based coloring (existing logic from growing.rs)
        if level < 20.0 {
            Color::Red
        } else if level < 40.0 {
            Color::Yellow
        } else {
            Color::Blue
        }
    }

    fn nutrient_color(&self, level: f32) -> Color {
        // Basic threshold-based coloring (existing logic from growing.rs)
        if level < 30.0 {
            Color::Red
        } else if level < 50.0 {
            Color::Yellow
        } else {
            Color::Green
        }
    }

    fn background_tint(&self, _stage: GrowthStage) -> Option<Color> {
        None // Not supported in 16-color mode
    }

    fn supports_rgb(&self) -> bool {
        false
    }
}

impl Default for Basic16Palette {
    fn default() -> Self {
        Self::new()
    }
}

/// 256-color indexed palette (FUTURE IMPLEMENTATION)
///
/// This palette is reserved for terminals that support 256-color mode but not RGB.
/// Currently falls back to Basic16Palette for all operations.
///
/// TODO: Implement 256-color indexed mapping from RGB values
/// TODO: Add terminal detection for 256-color support (via supports-color crate)
/// TODO: Create lookup table mapping RGB -> nearest 256-color index
#[derive(Debug)]
#[allow(dead_code)] // Intentionally unused - reserved for future implementation
pub struct Color256Palette;

impl Color256Palette {
    #[allow(dead_code)] // Intentionally unused - reserved for future implementation
    pub fn new() -> Self {
        Color256Palette
    }
}

impl ColorPalette for Color256Palette {
    fn flower_color(&self, variant: u8, intensity: FlowerIntensity, _stage: GrowthStage) -> Color {
        // TODO: Implement 256-color indexed mapping from RGB values
        // For now, fallback to Basic16
        Basic16Palette.flower_color(variant, intensity, _stage)
    }

    fn foliage_color(&self, variant: u8, health: f32, water: f32) -> Color {
        // TODO: Implement environmental modifiers with 256 colors
        Basic16Palette.foliage_color(variant, health, water)
    }

    fn trunk_color(&self, variant: u8, age_days: u32) -> Color {
        // TODO: Implement age-based color progression
        Basic16Palette.trunk_color(variant, age_days)
    }

    fn soil_color(&self, moisture: f32) -> Color {
        // TODO: Implement moisture-reactive soil colors
        Basic16Palette.soil_color(moisture)
    }

    fn water_color(&self, level: f32) -> Color {
        // TODO: Implement 256-color gradients
        Basic16Palette.water_color(level)
    }

    fn nutrient_color(&self, level: f32) -> Color {
        // TODO: Implement 256-color gradients
        Basic16Palette.nutrient_color(level)
    }

    fn background_tint(&self, _stage: GrowthStage) -> Option<Color> {
        None // Not supported in 256-color mode
    }

    fn supports_rgb(&self) -> bool {
        false
    }
}

impl Default for Color256Palette {
    fn default() -> Self {
        Self::new()
    }
}

/// True RGB 24-bit color palette (future implementation)
#[derive(Debug)]
pub struct TrueColorPalette;

impl TrueColorPalette {
    pub fn new() -> Self {
        TrueColorPalette
    }
}

impl ColorPalette for TrueColorPalette {
    fn flower_color(&self, variant: u8, intensity: FlowerIntensity, _stage: GrowthStage) -> Color {
        // 6 flower color variants with RGB gradients
        // Each variant has 4 intensity levels: Early → Developing → Peak → Harvest
        match (variant % 6, intensity) {
            // Variant 0 - Deep Purple (Indica)
            (0, FlowerIntensity::Early) => Color::Rgb(180, 120, 200),      // Lavender
            (0, FlowerIntensity::Developing) => Color::Rgb(140, 80, 180),  // Purple
            (0, FlowerIntensity::Peak) => Color::Rgb(120, 40, 160),        // Deep purple
            (0, FlowerIntensity::Harvest) => Color::Rgb(100, 20, 140),     // Royal purple (BOLD applied separately)

            // Variant 1 - Orange/Red (Sativa)
            (1, FlowerIntensity::Early) => Color::Rgb(255, 180, 100),      // Light orange
            (1, FlowerIntensity::Developing) => Color::Rgb(255, 140, 60),  // Orange
            (1, FlowerIntensity::Peak) => Color::Rgb(240, 100, 40),        // Deep orange
            (1, FlowerIntensity::Harvest) => Color::Rgb(220, 60, 20),      // Burnt orange/red

            // Variant 2 - Golden/Yellow
            (2, FlowerIntensity::Early) => Color::Rgb(255, 255, 150),      // Pale yellow
            (2, FlowerIntensity::Developing) => Color::Rgb(255, 220, 100), // Golden
            (2, FlowerIntensity::Peak) => Color::Rgb(240, 200, 60),        // Rich gold
            (2, FlowerIntensity::Harvest) => Color::Rgb(220, 180, 40),     // Deep amber

            // Variant 3 - Pink/Magenta
            (3, FlowerIntensity::Early) => Color::Rgb(255, 200, 220),      // Light pink
            (3, FlowerIntensity::Developing) => Color::Rgb(255, 150, 200), // Bright pink
            (3, FlowerIntensity::Peak) => Color::Rgb(240, 100, 180),       // Hot pink
            (3, FlowerIntensity::Harvest) => Color::Rgb(220, 60, 160),     // Deep magenta

            // Variant 4 - Blue/Teal (rare)
            (4, FlowerIntensity::Early) => Color::Rgb(150, 220, 230),      // Sky blue
            (4, FlowerIntensity::Developing) => Color::Rgb(100, 200, 220), // Teal
            (4, FlowerIntensity::Peak) => Color::Rgb(60, 180, 200),        // Deep teal
            (4, FlowerIntensity::Harvest) => Color::Rgb(40, 160, 180),     // Rich cyan

            // Variant 5 - White/Cream (high THC, frosty)
            (5, FlowerIntensity::Early) => Color::Rgb(240, 240, 220),      // Cream
            (5, FlowerIntensity::Developing) => Color::Rgb(255, 255, 240), // Bright white
            (5, FlowerIntensity::Peak) => Color::Rgb(255, 255, 255),       // Pure white
            (5, FlowerIntensity::Harvest) => Color::Rgb(240, 240, 255),    // Frosted white (DIM applied separately)

            _ => Color::Rgb(255, 100, 200), // Fallback pink
        }
    }

    fn foliage_color(&self, variant: u8, health: f32, water: f32) -> Color {
        // Base green colors (4 variants)
        let (mut r, mut g, mut b) = match variant % 4 {
            0 => (60, 140, 60),   // Forest green
            1 => (80, 180, 80),   // Bright green
            2 => (100, 200, 100), // Lime green
            3 => (40, 120, 70),   // Dark green
            _ => (60, 140, 60),   // Fallback
        };

        // Health-based modifications
        if health < 40.0 {
            // Critical: Brown tint (dying plant)
            r = 120;
            g = 100;
            b = 60;
        } else if health < 60.0 {
            // Poor: Reduce green, add yellow (stress)
            g = (g as f32 * 0.7) as u8;
            r = (r as f32 * 1.3).min(255.0) as u8;
        } else if health < 80.0 {
            // Fair: Slightly reduce green
            g = (g as f32 * 0.8) as u8;
        }
        // Good/Excellent: No modification

        // Water level modifications
        if water < 30.0 {
            // Drought: Reduce saturation (wilting)
            let avg = ((r as u16 + g as u16 + b as u16) / 3) as u8;
            r = (r as f32 * 0.6 + avg as f32 * 0.4) as u8;
            g = (g as f32 * 0.6 + avg as f32 * 0.4) as u8;
            b = (b as f32 * 0.6 + avg as f32 * 0.4) as u8;
        } else if water > 80.0 {
            // Well-hydrated: Increase brightness
            r = (r as f32 * 1.1).min(255.0) as u8;
            g = (g as f32 * 1.1).min(255.0) as u8;
            b = (b as f32 * 1.1).min(255.0) as u8;
        }

        Color::Rgb(r, g, b)
    }

    fn trunk_color(&self, variant: u8, age_days: u32) -> Color {
        // Base trunk colors (realistic wood tones)
        let (mut r, mut g, mut b) = match variant % 3 {
            0 => (139, 90, 60),  // Light wood (tan/beige)
            1 => (101, 67, 33),  // Medium wood (brown)
            2 => (70, 50, 30),   // Dark wood (dark brown)
            _ => (101, 67, 33),  // Fallback
        };

        // Age-based weathering (young stem → mature woody bark)
        if age_days <= 20 {
            // Days 1-20: Young green stem
            // Add green tint to make it look like fresh stem
            g = (g as f32 * 1.3).min(255.0) as u8;
            b = (b as f32 * 0.9) as u8;
        } else if age_days <= 50 {
            // Days 21-50: Maturing (transition to wood)
            r = (r as f32 + 10.0).min(255.0) as u8;
            g = (g as f32 - 10.0).max(0.0) as u8;
        } else {
            // Days 51+: Mature woody bark
            r = (r as f32 + 20.0).min(255.0) as u8;
            g = (g as f32 - 20.0).max(0.0) as u8;
        }

        Color::Rgb(r, g, b)
    }

    fn soil_color(&self, moisture: f32) -> Color {
        // Moisture-reactive soil colors
        if moisture > 70.0 {
            // Wet soil: Dark, rich
            Color::Rgb(80, 60, 40)
        } else if moisture > 40.0 {
            // Moist soil: Medium brown
            Color::Rgb(120, 90, 60)
        } else {
            // Dry soil: Light, dusty
            Color::Rgb(160, 130, 90)
        }
    }

    fn water_color(&self, level: f32) -> Color {
        // Smooth RGB gradient: Red (0%) → Orange → Yellow → Cyan → Blue (100%)
        // Represents water level visually: critical → low → good → abundant
        let level = level.clamp(0.0, 100.0);

        if level < 20.0 {
            // 0-20%: Red (critical)
            let t = level / 20.0;
            Color::Rgb(
                255,
                (60.0 * t) as u8,  // 0 → 60
                0,
            )
        } else if level < 40.0 {
            // 20-40%: Red → Yellow (low)
            let t = (level - 20.0) / 20.0;
            Color::Rgb(
                255,
                (60.0 + 195.0 * t) as u8,  // 60 → 255
                0,
            )
        } else if level < 60.0 {
            // 40-60%: Yellow → Cyan (adequate)
            let t = (level - 40.0) / 20.0;
            Color::Rgb(
                (255.0 * (1.0 - t)) as u8,  // 255 → 0
                255,
                (255.0 * t) as u8,  // 0 → 255
            )
        } else {
            // 60-100%: Cyan → Deep Blue (excellent)
            let t = (level - 60.0) / 40.0;
            Color::Rgb(
                0,
                (255.0 * (1.0 - t * 0.7)) as u8,  // 255 → ~75
                255,
            )
        }
    }

    fn nutrient_color(&self, level: f32) -> Color {
        // Smooth RGB gradient: Red (0%) → Orange → Yellow → Yellow-Green → Green (100%)
        // Represents nutrient level: critical → low → good → excellent
        let level = level.clamp(0.0, 100.0);

        if level < 30.0 {
            // 0-30%: Red → Orange (critical)
            let t = level / 30.0;
            Color::Rgb(
                255,
                (120.0 * t) as u8,  // 0 → 120
                0,
            )
        } else if level < 50.0 {
            // 30-50%: Orange → Yellow (low)
            let t = (level - 30.0) / 20.0;
            Color::Rgb(
                255,
                (120.0 + 135.0 * t) as u8,  // 120 → 255
                0,
            )
        } else if level < 75.0 {
            // 50-75%: Yellow → Yellow-Green (good)
            let t = (level - 50.0) / 25.0;
            Color::Rgb(
                (255.0 * (1.0 - t * 0.5)) as u8,  // 255 → 127
                255,
                (100.0 * t) as u8,  // 0 → 100
            )
        } else {
            // 75-100%: Yellow-Green → Pure Green (excellent)
            let t = (level - 75.0) / 25.0;
            Color::Rgb(
                (127.0 * (1.0 - t)) as u8,  // 127 → 0
                255,
                (100.0 + 55.0 * t) as u8,  // 100 → 155
            )
        }
    }

    fn background_tint(&self, stage: GrowthStage) -> Option<Color> {
        // Subtle background tints for each growth stage
        // Very faint to not overwhelm the plant visual
        Some(match stage {
            GrowthStage::Seed | GrowthStage::Germination => Color::Rgb(5, 10, 5),   // Very faint green
            GrowthStage::Seedling => Color::Rgb(5, 10, 5),                           // Very faint green
            GrowthStage::Vegetative => Color::Rgb(10, 20, 10),                       // Faint green (growth)
            GrowthStage::PreFlower => Color::Rgb(20, 20, 5),                         // Yellow tint (transition)
            GrowthStage::Flowering => Color::Rgb(15, 5, 20),                         // Purple tint (flowers)
            GrowthStage::ReadyToHarvest => Color::Rgb(25, 20, 5),                    // Golden tint (ripe)
        })
    }

    fn supports_rgb(&self) -> bool {
        true
    }
}

impl Default for TrueColorPalette {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert HSV to RGB color
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let h_prime = (h % 360.0) / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::Rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

/// Rainbow Palette - HSV cycling colors, energetic and vibrant
/// Note: Currently static (frame=0), will be animated in Phase 2.5
#[derive(Debug)]
pub struct RainbowPalette;

impl ColorPalette for RainbowPalette {
    fn flower_color(&self, variant: u8, intensity: FlowerIntensity, _stage: GrowthStage) -> Color {
        // Each variant gets a different hue offset (60° apart for variety)
        let hue_offset = (variant as f32) * 60.0;

        // TODO: Add frame-based cycling in future - for now static hue
        let hue = hue_offset;

        // Intensity affects saturation and value
        let (s, v) = match intensity {
            FlowerIntensity::Early => (0.5, 0.7),
            FlowerIntensity::Developing => (0.7, 0.9),
            FlowerIntensity::Peak => (1.0, 1.0),
            FlowerIntensity::Harvest => (1.0, 1.0),
        };

        hsv_to_rgb(hue, s, v)
    }

    fn foliage_color(&self, variant: u8, _health: f32, _water: f32) -> Color {
        // Green base (120°) with variants
        let hue_offset = (variant as f32) * 90.0;
        let hue = (120.0 + hue_offset) % 360.0;
        hsv_to_rgb(hue, 0.6, 0.8)
    }

    fn trunk_color(&self, variant: u8, _age: u32) -> Color {
        // Brown/orange base (30°) with variants
        let hue_offset = (variant as f32) * 120.0;
        let hue = (30.0 + hue_offset) % 360.0;
        hsv_to_rgb(hue, 0.5, 0.6)
    }

    fn soil_color(&self, moisture: f32) -> Color {
        // Brown hue (30°), varies value with moisture
        let v = 0.2 + (moisture / 100.0 * 0.4);  // 0.2-0.6
        hsv_to_rgb(30.0, 0.5, v)
    }

    fn water_color(&self, level: f32) -> Color {
        // Cyan to blue gradient (180°-240°)
        let hue = 180.0 + (level / 100.0 * 60.0);
        hsv_to_rgb(hue, 0.8, 0.9)
    }

    fn nutrient_color(&self, level: f32) -> Color {
        // Yellow to green gradient (60°-120°)
        let hue = 60.0 + (level / 100.0 * 60.0);
        hsv_to_rgb(hue, 0.7, 0.9)
    }

    fn background_tint(&self, _stage: GrowthStage) -> Option<Color> {
        Some(Color::Rgb(15, 10, 20))  // Subtle purple tint
    }

    fn supports_rgb(&self) -> bool {
        true
    }
}

/// Zen Palette - Minimalist, soft colors, calming aesthetic
#[derive(Debug)]
pub struct ZenPalette;

impl ColorPalette for ZenPalette {
    fn flower_color(&self, _variant: u8, intensity: FlowerIntensity, _stage: GrowthStage) -> Color {
        // Soft pastel colors - lavender, pale pink, cream
        match intensity {
            FlowerIntensity::Early => Color::Rgb(200, 200, 220),     // Pale lavender
            FlowerIntensity::Developing => Color::Rgb(220, 200, 210), // Pale pink
            FlowerIntensity::Peak => Color::Rgb(230, 220, 210),      // Cream
            FlowerIntensity::Harvest => Color::Rgb(240, 230, 220),   // Warm cream
        }
    }

    fn foliage_color(&self, _variant: u8, health: f32, _water: f32) -> Color {
        // Soft sage greens - desaturated, calming
        if health > 70.0 {
            Color::Rgb(140, 160, 140)  // Healthy sage
        } else if health > 40.0 {
            Color::Rgb(160, 170, 150)  // Faded sage
        } else {
            Color::Rgb(180, 180, 170)  // Very pale (stressed)
        }
    }

    fn trunk_color(&self, _variant: u8, _age: u32) -> Color {
        Color::Rgb(160, 140, 120)  // Soft tan
    }

    fn soil_color(&self, moisture: f32) -> Color {
        if moisture > 60.0 {
            Color::Rgb(130, 120, 110)  // Soft dark brown
        } else {
            Color::Rgb(180, 170, 150)  // Light beige
        }
    }

    fn water_color(&self, level: f32) -> Color {
        // Soft blue gradient
        let t = (level / 100.0).clamp(0.0, 1.0);
        Color::Rgb(
            (180.0 + 40.0 * t) as u8,   // 180 → 220
            (200.0 + 30.0 * t) as u8,   // 200 → 230
            (220.0 + 20.0 * t) as u8,   // 220 → 240
        )
    }

    fn nutrient_color(&self, level: f32) -> Color {
        // Soft sage gradient
        let t = (level / 100.0).clamp(0.0, 1.0);
        Color::Rgb(
            (180.0 - 40.0 * t) as u8,   // 180 → 140
            (200.0 - 20.0 * t) as u8,   // 200 → 180
            (160.0 - 20.0 * t) as u8,   // 160 → 140
        )
    }

    fn background_tint(&self, _stage: GrowthStage) -> Option<Color> {
        Some(Color::Rgb(10, 12, 10))  // Very subtle gray-green
    }

    fn supports_rgb(&self) -> bool {
        true
    }
}

/// Matrix Palette - Green monochrome, retro hacker aesthetic
#[derive(Debug)]
pub struct MatrixPalette;

impl ColorPalette for MatrixPalette {
    fn flower_color(&self, _variant: u8, intensity: FlowerIntensity, _stage: GrowthStage) -> Color {
        // Varying shades of green based on intensity
        match intensity {
            FlowerIntensity::Early => Color::Rgb(0, 180, 0),
            FlowerIntensity::Developing => Color::Rgb(0, 220, 0),
            FlowerIntensity::Peak => Color::Rgb(0, 255, 0),      // Bright matrix green
            FlowerIntensity::Harvest => Color::Rgb(100, 255, 100), // Lighter (glowing)
        }
    }

    fn foliage_color(&self, _variant: u8, health: f32, _water: f32) -> Color {
        // Green monochrome - darker for lower health
        let brightness = (120.0 + (health / 100.0) * 135.0) as u8;  // 120-255
        Color::Rgb(0, brightness, 0)
    }

    fn trunk_color(&self, _variant: u8, age: u32) -> Color {
        // Dark green trunk - gets brighter with age
        let base = 60 + (age.min(90) / 3) as u8;  // 60-90
        Color::Rgb(0, base, 0)
    }

    fn soil_color(&self, moisture: f32) -> Color {
        // Very dark green - almost black when dry
        let g = (20.0 + moisture * 0.5) as u8;  // 20-70
        Color::Rgb(0, g, 0)
    }

    fn water_color(&self, level: f32) -> Color {
        // Green gradient
        let g = (100.0 + level * 1.55) as u8;  // 100-255
        Color::Rgb(0, g, 0)
    }

    fn nutrient_color(&self, level: f32) -> Color {
        // Lime green gradient
        let g = (150.0 + level * 1.05) as u8;  // 150-255
        Color::Rgb(50, g, 0)
    }

    fn background_tint(&self, _stage: GrowthStage) -> Option<Color> {
        Some(Color::Rgb(0, 5, 0))  // Very dark green
    }

    fn supports_rgb(&self) -> bool {
        true
    }
}

/// Create appropriate color palette based on terminal capabilities and visual mode
pub fn create_palette(supports_truecolor: bool, visual_mode: crate::ui::visual_mode::VisualMode) -> Box<dyn ColorPalette> {
    if !supports_truecolor {
        // 16-color mode - only Normal mode available
        return Box::new(Basic16Palette::new());
    }

    // TrueColor mode - return palette based on visual mode
    match visual_mode {
        crate::ui::visual_mode::VisualMode::Normal => Box::new(TrueColorPalette::new()),
        crate::ui::visual_mode::VisualMode::Zen => Box::new(ZenPalette),
        crate::ui::visual_mode::VisualMode::Rainbow => Box::new(RainbowPalette),
        crate::ui::visual_mode::VisualMode::Matrix => Box::new(MatrixPalette),
    }
}
