use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::app::App;
use crate::ascii::{
    get_border_decoration, get_nutrient_sparkles, get_plant_ascii, get_water_drops,
};
use crate::domain::Plant;
use crate::ui::colors::FlowerIntensity;

// Environmental thresholds for visual feedback
const TEMP_OPTIMAL_MIN: f32 = 20.0;
const TEMP_OPTIMAL_MAX: f32 = 28.0;
const TEMP_ACCEPTABLE_MIN: f32 = 18.0;
const TEMP_ACCEPTABLE_MAX: f32 = 30.0;

const HUMIDITY_OPTIMAL_MIN: f32 = 50.0;
const HUMIDITY_OPTIMAL_MAX: f32 = 70.0;
const HUMIDITY_ACCEPTABLE_MIN: f32 = 40.0;
const HUMIDITY_ACCEPTABLE_MAX: f32 = 80.0;

const GROWTH_GOOD_THRESHOLD: f32 = 60.0;
const GROWTH_FAIR_THRESHOLD: f32 = 30.0;

// Flower intensity day thresholds
const FLOWER_DEVELOPING_DAY: u32 = 61;
const FLOWER_PEAK_DAY: u32 = 71;

/// Applies a breathing effect to a color by adjusting brightness
/// In RGB mode, multiplies RGB values by the factor (0.8-1.0 range for subtle effect)
/// In 16-color mode, returns the color unchanged (no breathing in basic mode)
fn apply_breathing(color: Color, factor: f32) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            // Apply brightness factor to RGB values
            Color::Rgb(
                ((r as f32 * factor).min(255.0)) as u8,
                ((g as f32 * factor).min(255.0)) as u8,
                ((b as f32 * factor).min(255.0)) as u8,
            )
        }
        // For 16-color mode, return unchanged (no breathing effect)
        other => other,
    }
}

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    if let Some(ref plant) = app.current_plant {
        render_plant(f, plant, area, app.animation_frame, app);
    } else {
        render_no_plant(f, area);
    }
}

fn render_plant(f: &mut Frame, plant: &Plant, area: Rect, frame: usize, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70), // Left: Plant + resources
            Constraint::Percentage(30), // Right: Strain info
        ])
        .split(area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Plant display
            Constraint::Length(11), // Resources (3 rows)
            Constraint::Length(3),  // Controls
        ])
        .split(main_chunks[0]);

    // Detect layout mode from terminal size
    let layout_mode = crate::ui::layout::LayoutMode::from_terminal_size(area.width, area.height);

    // Animated header with speed indicator
    let decoration = get_border_decoration(frame);
    let speed_indicator = if frame % 4 < 2 { ">" } else { "<" };
    let header = Paragraph::new(format!(
        "{} GanjaTUI [{}] - Day {} | {} | {} {} [By ZeD {}]",
        decoration,
        layout_mode.indicator(),
        plant.days_alive,
        plant.stage.as_str(),
        app.visual_mode.name(),
        decoration,
        speed_indicator
    ))
    .block(Block::default().borders(Borders::ALL))
    .alignment(Alignment::Center)
    .style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    f.render_widget(header, chunks[0]);

    // Animated plant display - procedurally generated based on plant ID
    let seed = plant.id.as_u128() as u64;
    let plant_ascii = get_plant_ascii(plant.stage, plant.days_alive, seed, frame);

    // Determine color variants based on genetics (seed) - each plant has unique colors!
    let flower_color_variant = (seed % 6) as u8;
    let foliage_color_variant = ((seed / 6) % 4) as u8;
    let trunk_color_variant = ((seed / 24) % 3) as u8;

    // Calculate flower intensity based on growth stage AND days alive for progression
    // Days 49-60: Early, 61-70: Developing, 71-85: Peak, 86+: Harvest
    let (flower_intensity_1, flower_intensity_2, flower_intensity_3) = match plant.stage {
        crate::domain::GrowthStage::Flowering => {
            if plant.days_alive < FLOWER_DEVELOPING_DAY {
                (FlowerIntensity::Early, FlowerIntensity::Early, FlowerIntensity::Developing)
            } else if plant.days_alive < FLOWER_PEAK_DAY {
                (FlowerIntensity::Developing, FlowerIntensity::Developing, FlowerIntensity::Peak)
            } else {
                // Late flowering (Peak intensity)
                (FlowerIntensity::Peak, FlowerIntensity::Peak, FlowerIntensity::Peak)
            }
        }
        crate::domain::GrowthStage::ReadyToHarvest => {
            (FlowerIntensity::Harvest, FlowerIntensity::Harvest, FlowerIntensity::Harvest)
        }
        _ => {
            // PreFlower or earlier
            (FlowerIntensity::Early, FlowerIntensity::Early, FlowerIntensity::Early)
        }
    };

    // Get colors from palette (uses RGB in truecolor mode, 16-color fallback otherwise)
    let palette = &app.color_palette;

    // Foliage color with environmental modifiers (health, water level)
    let health_percent = match plant.health {
        crate::domain::HealthStatus::Excellent => 100.0,
        crate::domain::HealthStatus::Good => 80.0,
        crate::domain::HealthStatus::Fair => 60.0,
        crate::domain::HealthStatus::Poor => 40.0,
        crate::domain::HealthStatus::Critical => 20.0,
    };
    let base_foliage_color = palette.foliage_color(foliage_color_variant, health_percent, plant.water_level);

    // Apply breathing effect to foliage and flowers (12.5% amplitude for visible pulsing)
    // Mode-specific breathing speeds for different aesthetics
    let breath_speed = match app.visual_mode {
        crate::ui::visual_mode::VisualMode::Normal => 0.05,   // Normal speed
        crate::ui::visual_mode::VisualMode::Zen => 0.02,      // Slower (calming)
        crate::ui::visual_mode::VisualMode::Rainbow => 0.08,  // Faster (energetic)
        crate::ui::visual_mode::VisualMode::Matrix => 0.06,   // Medium-fast (digital)
    };
    let breath_factor = 0.875 + ((frame as f32 * breath_speed).sin() * 0.125); // 0.75-1.00 range (12.5% amplitude)
    let foliage_color = apply_breathing(base_foliage_color, breath_factor);

    // Flower colors with intensity progression + breathing effect
    let base_flower_color_1 = palette.flower_color(flower_color_variant, flower_intensity_1, plant.stage);
    let base_flower_color_2 = palette.flower_color(flower_color_variant, flower_intensity_2, plant.stage);
    let base_flower_color_3 = palette.flower_color(flower_color_variant, flower_intensity_3, plant.stage);

    let flower_color_1 = apply_breathing(base_flower_color_1, breath_factor);
    let flower_color_2 = apply_breathing(base_flower_color_2, breath_factor);
    let flower_color_3 = apply_breathing(base_flower_color_3, breath_factor);

    // Trunk color with age progression
    let trunk_color = palette.trunk_color(trunk_color_variant, plant.days_alive);

    // Soil color (moisture-reactive)
    let soil_color = palette.soil_color(plant.water_level);

    // Build content lines first with colorization
    let mut content_lines = vec![];
    for line in plant_ascii {
        // Colorize each character based on type and growth stage
        let mut spans = vec![];
        let mut current_chars = String::new();
        let mut current_color = None;

        for ch in line.chars() {
            let color = match ch {
                // Trunk characters - varied wood tones
                '|' | '!' | 'I' | '║' => Some(trunk_color),

                // Branch characters - varied green tones
                '/' | '\\' | '_' | '=' => match plant.stage {
                    crate::domain::GrowthStage::Seed | crate::domain::GrowthStage::Germination => {
                        Some(Color::DarkGray)
                    }
                    crate::domain::GrowthStage::Seedling => Some(Color::Green),
                    _ => Some(foliage_color),
                },

                // Flower/bud characters - SUPER VIBRANT when ready!
                '*' => {
                    match plant.stage {
                        crate::domain::GrowthStage::Flowering => Some(flower_color_1),
                        crate::domain::GrowthStage::ReadyToHarvest => Some(flower_color_3), // VIBRANT!
                        _ => Some(foliage_color),
                    }
                }
                'o' => {
                    match plant.stage {
                        crate::domain::GrowthStage::PreFlower => Some(Color::Yellow),
                        crate::domain::GrowthStage::Flowering => Some(flower_color_1),
                        crate::domain::GrowthStage::ReadyToHarvest => Some(flower_color_3), // VIBRANT!
                        _ => Some(foliage_color),
                    }
                }
                'O' => {
                    match plant.stage {
                        crate::domain::GrowthStage::Flowering => Some(flower_color_2),
                        crate::domain::GrowthStage::ReadyToHarvest => Some(flower_color_3), // VIBRANT!
                        _ => Some(foliage_color),
                    }
                }
                '@' | '#' => {
                    match plant.stage {
                        crate::domain::GrowthStage::Flowering => Some(flower_color_2),
                        crate::domain::GrowthStage::ReadyToHarvest => Some(flower_color_3), // VIBRANT!
                        _ => Some(foliage_color),
                    }
                }

                // Foliage - varied greens
                ':' => Some(foliage_color),

                // Soil - moisture-reactive
                '~' => Some(soil_color),

                // Spaces and other characters - no color
                _ => None,
            };

            // If color changed, flush current buffer
            if current_color != color && !current_chars.is_empty() {
                if let Some(c) = current_color {
                    spans.push(Span::styled(current_chars.clone(), Style::default().fg(c)));
                } else {
                    spans.push(Span::raw(current_chars.clone()));
                }
                current_chars.clear();
            }

            current_chars.push(ch);
            current_color = color;
        }

        // Flush remaining characters
        if !current_chars.is_empty() {
            if let Some(c) = current_color {
                spans.push(Span::styled(current_chars, Style::default().fg(c)));
            } else {
                spans.push(Span::raw(current_chars));
            }
        }

        content_lines.push(Line::from(spans));
    }

    // Fixed positioning - add padding at TOP to push plant to bottom
    // This keeps the soil line always at the same position
    let available_height = chunks[1].height.saturating_sub(2) as usize; // Subtract borders
    let content_height = content_lines.len();
    let padding_top = available_height.saturating_sub(content_height);

    let mut plant_lines = vec![];
    for _ in 0..padding_top {
        plant_lines.push(Line::from(""));
    }
    plant_lines.extend(content_lines);

    // Create plant display with optional background tint
    let mut plant_style = Style::default();
    if let Some(bg_color) = palette.background_tint(plant.stage) {
        plant_style = plant_style.bg(bg_color);
    }

    let plant_display = Paragraph::new(plant_lines)
        .block(Block::default().borders(Borders::ALL).title("[ Plant ]"))
        .alignment(Alignment::Center)
        .style(plant_style);
    f.render_widget(plant_display, chunks[1]);

    // Dynamic metrics - 3 rows of gauges (things that change frequently)
    let resources_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Row 1: Water, Nutrients, Growth Progress
            Constraint::Length(3), // Row 2: Temperature, Humidity, Roots/Canopy
            Constraint::Length(3), // Row 3: Health
        ])
        .split(chunks[2]);

    let row1_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(resources_rows[0]);

    let row2_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(resources_rows[1]);

    // Water gauge with animated drops - RGB gradient in truecolor mode
    let water_color = palette.water_color(plant.water_level);

    let water_drops = get_water_drops(frame);
    let water_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Water{}", water_drops)),
        )
        .gauge_style(Style::default().fg(water_color))
        .percent(plant.water_level as u16)
        .label(format!("{:.0}%", plant.water_level));
    f.render_widget(water_gauge, row1_chunks[0]);

    // Nutrient gauge with animated sparkles - RGB gradient in truecolor mode
    let nutrient_color = palette.nutrient_color(plant.nutrient_level);

    let sparkles = get_nutrient_sparkles(frame);
    let nutrient_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("NPK{}", sparkles)),
        )
        .gauge_style(Style::default().fg(nutrient_color))
        .percent(plant.nutrient_level as u16)
        .label(format!("{:.0}%", plant.nutrient_level));
    f.render_widget(nutrient_gauge, row1_chunks[1]);

    // Growth Progress gauge - % to next stage (changes every day!)
    let (current_day, next_stage_day, stage_name): (u32, u32, &str) = match plant.stage {
        crate::domain::GrowthStage::Seed | crate::domain::GrowthStage::Germination => {
            (plant.days_alive, 11, "Vegetative")
        }
        crate::domain::GrowthStage::Seedling => (plant.days_alive, 11, "Vegetative"),
        crate::domain::GrowthStage::Vegetative => (plant.days_alive, 41, "Pre-Flower"),
        crate::domain::GrowthStage::PreFlower => (plant.days_alive, 49, "Flowering"),
        crate::domain::GrowthStage::Flowering => (plant.days_alive, 86, "Harvest"),
        crate::domain::GrowthStage::ReadyToHarvest => (86, 86, "Ready!"),
    };
    let progress_percent = if plant.stage == crate::domain::GrowthStage::ReadyToHarvest {
        100
    } else {
        ((current_day as f32 / next_stage_day as f32) * 100.0).min(100.0) as u16
    };
    let days_left = next_stage_day.saturating_sub(current_day);
    let progress_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("→ {}", stage_name)),
        )
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(progress_percent)
        .label(format!("{}d left", days_left));
    f.render_widget(progress_gauge, row1_chunks[2]);

    // Temperature gauge - oscillates realistically (changes visibly!)
    let temp_percent = ((plant.temperature - TEMP_OPTIMAL_MIN) / (TEMP_OPTIMAL_MAX - TEMP_OPTIMAL_MIN) * 100.0)
        .max(0.0)
        .min(100.0) as u16;
    let temp_color = if plant.temperature >= TEMP_OPTIMAL_MIN && plant.temperature <= TEMP_OPTIMAL_MAX {
        Color::Green
    } else if plant.temperature >= TEMP_ACCEPTABLE_MIN && plant.temperature <= TEMP_ACCEPTABLE_MAX {
        Color::Yellow
    } else {
        Color::Red
    };
    let temp_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Temperature"))
        .gauge_style(Style::default().fg(temp_color))
        .percent(temp_percent)
        .label(format!("{:.1}°C", plant.temperature));
    f.render_widget(temp_gauge, row2_chunks[0]);

    // Humidity gauge - varies with watering (dynamic!)
    let humid_percent = plant.humidity as u16;
    let humid_color = if plant.humidity >= HUMIDITY_OPTIMAL_MIN && plant.humidity <= HUMIDITY_OPTIMAL_MAX {
        Color::Cyan
    } else if plant.humidity >= HUMIDITY_ACCEPTABLE_MIN && plant.humidity <= HUMIDITY_ACCEPTABLE_MAX {
        Color::Yellow
    } else {
        Color::Red
    };
    let humid_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Humidity"))
        .gauge_style(Style::default().fg(humid_color))
        .percent(humid_percent)
        .label(format!("{:.0}%", plant.humidity));
    f.render_widget(humid_gauge, row2_chunks[1]);

    // Roots & Canopy development
    let growth_color = if plant.root_development >= GROWTH_GOOD_THRESHOLD {
        Color::Green
    } else if plant.root_development >= GROWTH_FAIR_THRESHOLD {
        Color::Yellow
    } else {
        Color::Red
    };
    let growth_gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Root/Canopy")),
        )
        .gauge_style(Style::default().fg(growth_color))
        .percent(((plant.root_development + plant.canopy_density) / 2.0) as u16)
        .label(format!(
            "R{:.0}/C{:.0}",
            plant.root_development, plant.canopy_density
        ));
    f.render_widget(growth_gauge, row2_chunks[2]);

    // Health gauge - overall plant health
    let (health_percent, health_color, health_label) = match plant.health {
        crate::domain::HealthStatus::Excellent => (100, Color::Green, "Excellent ★"),
        crate::domain::HealthStatus::Good => (75, Color::Green, "Good"),
        crate::domain::HealthStatus::Fair => (50, Color::Yellow, "Fair"),
        crate::domain::HealthStatus::Poor => (25, Color::LightRed, "Poor ⚠"),
        crate::domain::HealthStatus::Critical => (10, Color::Red, "CRITICAL ⚠⚠"),
    };

    let health_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Health"))
        .gauge_style(Style::default().fg(health_color))
        .percent(health_percent)
        .label(health_label);
    f.render_widget(health_gauge, resources_rows[2]);

    // Controls with auto-harvest mode indicator
    let auto_mode_indicator = if app.auto_harvest {
        " | AUTO ✓ "
    } else {
        ""
    };

    let controls = if plant.stage == crate::domain::GrowthStage::ReadyToHarvest {
        format!("** [h] HARVEST **  [a] Auto{}  [v] Mode  [s] Stats  [q] Quit", auto_mode_indicator)
    } else {
        format!("[h] Harvest (ready)  [a] Auto{}  [v] Mode  [s] Stats  [q] Quit", auto_mode_indicator)
    };

    let controls_style = if plant.stage == crate::domain::GrowthStage::ReadyToHarvest {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let controls_widget = Paragraph::new(controls)
        .block(Block::default().borders(Borders::ALL).title("Controls"))
        .style(controls_style)
        .alignment(Alignment::Center);
    f.render_widget(controls_widget, chunks[3]);

    // Strain Info Panel (right side)
    let strain_info_lines = if let Some(ref strain_info) = plant.genetics.strain_info {
        vec![
            Line::from(Span::styled(
                strain_info.name.clone(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!("Type: {}", strain_info.strain_type),
                Style::default().fg(Color::Yellow),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Genetics:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(strain_info.genetics.clone()),
            Line::from(""),
            Line::from(Span::styled(
                "Cannabinoids:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("THC: {:.1}%", plant.genetics.thc_percent)),
            Line::from(format!("CBD: {:.1}%", plant.genetics.cbd_percent)),
            Line::from(""),
            Line::from(Span::styled(
                "Characteristics:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("Difficulty: {}", strain_info.difficulty)),
            Line::from(format!("Yield: {}", strain_info.yield_potential)),
            Line::from(format!("Flowering: {} days", strain_info.flowering_time)),
            Line::from(""),
            Line::from(Span::styled(
                "Terpenes:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(strain_info.dominant_terpenes.join(", ")),
            Line::from(""),
            Line::from(Span::styled(
                "Aroma:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(strain_info.aroma.join(", ")),
            Line::from(""),
            Line::from(Span::styled(
                "Effects:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(strain_info.effects.join(", ")),
        ]
    } else {
        vec![
            Line::from(Span::styled(
                plant.strain_name.clone(),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("No strain data available"),
            Line::from(""),
            Line::from(Span::styled(
                "Cannabinoids:",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(format!("THC: {:.1}%", plant.genetics.thc_percent)),
            Line::from(format!("CBD: {:.1}%", plant.genetics.cbd_percent)),
        ]
    };

    let strain_info_widget = Paragraph::new(strain_info_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("[ Strain Info ]"),
        )
        .alignment(Alignment::Left)
        .style(Style::default());
    f.render_widget(strain_info_widget, main_chunks[1]);
}

fn render_no_plant(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "No plant currently growing",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Press '4' to go to Storage and plant a new seed"),
        Line::from(""),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("[ Growing Room ]"),
        )
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}
