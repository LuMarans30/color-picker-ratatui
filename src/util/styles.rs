use ratatui::style::{Color, Style};

pub struct Styles;

impl Styles {
    /// Get border color based on focus state and validity
    pub fn border_color(focused: bool, is_valid: Option<bool>) -> Color {
        match (focused, is_valid) {
            (true, _) => Color::Cyan,
            (false, Some(true)) => Color::Green,
            (false, Some(false)) => Color::Red,
            (false, None) => Color::Reset,
        }
    }

    /// Get button colors based on state
    pub fn button_colors(selected: bool) -> (Color, Color) {
        if selected {
            (Color::Blue, Color::White)
        } else {
            (Color::DarkGray, Color::Gray)
        }
    }

    /// Common modal background style
    pub fn modal_background() -> Style {
        Style::default().bg(Color::DarkGray)
    }

    /// Focus border style
    pub fn focus_border(focused: bool) -> Style {
        Style::default().fg(if focused { Color::Cyan } else { Color::Reset })
    }
}
