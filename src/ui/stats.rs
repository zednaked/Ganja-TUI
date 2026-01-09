use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "GANJATUI - Cannabis Growth Simulator",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Statistics:",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(format!("Total Harvests: {}", app.total_harvests)),
    ];

    // Calculate and show aggregate statistics
    if !app.harvest_history.is_empty() {
        let total_count = app.harvest_history.len() as f32;

        let avg_yield: f32 = app.harvest_history.iter()
            .map(|h| h.weight_grams)
            .sum::<f32>() / total_count;

        let avg_quality: f32 = app.harvest_history.iter()
            .map(|h| h.quality_score)
            .sum::<f32>() / total_count;

        let avg_thc: f32 = app.harvest_history.iter()
            .map(|h| h.thc_percent)
            .sum::<f32>() / total_count;

        let avg_cbd: f32 = app.harvest_history.iter()
            .map(|h| h.cbd_percent)
            .sum::<f32>() / total_count;

        let total_yield: f32 = app.harvest_history.iter()
            .map(|h| h.weight_grams)
            .sum();

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("Average Yield: "),
            Span::styled(
                format!("{:.1}g", avg_yield),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | Quality: "),
            Span::styled(
                format!("{:.0}%", avg_quality),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::raw("Average THC: "),
            Span::styled(
                format!("{:.1}%", avg_thc),
                Style::default().fg(Color::Magenta),
            ),
            Span::raw(" | CBD: "),
            Span::styled(
                format!("{:.1}%", avg_cbd),
                Style::default().fg(Color::Blue),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::raw("Total Yield All-Time: "),
            Span::styled(
                format!("{:.1}g", total_yield),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    lines.push(Line::from(""));

    // Show last 5 harvests with detailed info
    if !app.harvest_history.is_empty() {
        lines.push(Line::from(Span::styled(
            "Recent Harvests:",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        let recent = app.harvest_history.iter().rev().take(5);
        for (i, harvest) in recent.enumerate() {
            // Harvest number and strain name
            lines.push(Line::from(vec![
                Span::raw(format!("{}. ", app.harvest_history.len() - i)),
                Span::styled(
                    &harvest.strain_name,
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                ),
            ]));

            // Yield and quality on one line
            let quality_color = if harvest.quality_score >= 90.0 {
                Color::Green
            } else if harvest.quality_score >= 75.0 {
                Color::Yellow
            } else {
                Color::Red
            };

            lines.push(Line::from(vec![
                Span::raw("   Yield: "),
                Span::styled(
                    format!("{:.1}g", harvest.weight_grams),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" | Quality: "),
                Span::styled(
                    format!("{:.0}%", harvest.quality_score),
                    Style::default().fg(quality_color).add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(" | Day {}", harvest.harvest_day)),
            ]));

            // Cannabinoids on another line
            lines.push(Line::from(vec![
                Span::raw("   THC: "),
                Span::styled(
                    format!("{:.1}%", harvest.thc_percent),
                    Style::default().fg(Color::Magenta),
                ),
                Span::raw(" | CBD: "),
                Span::styled(
                    format!("{:.1}%", harvest.cbd_percent),
                    Style::default().fg(Color::Blue),
                ),
            ]));

            lines.push(Line::from("")); // Spacing between harvests
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "About:",
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));
    lines.push(Line::from("A procedural cannabis growth simulator"));
    lines.push(Line::from("Each plant is unique with different genetics"));
    lines.push(Line::from("by ZeD - zednaked@gmail.com"));
    lines.push(Line::from(""));
    lines.push(Line::from("Press [1] to return to Growing Room"));

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("[ Statistics & About ]"))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
