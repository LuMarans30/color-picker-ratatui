use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Modifier, Style},
    widgets::Widget,
};

#[derive(Debug, Default, Clone)]
pub struct ColorInput {
    pub input: String,
    pub cursor_pos: usize,
    pub focused: bool,
}

impl ColorInput {
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char(c) if c.is_ascii_hexdigit() && self.input.len() < 6 => {
                let c = c.to_ascii_uppercase();
                self.input.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
            }
            KeyCode::Backspace if self.cursor_pos > 0 => {
                self.input.remove(self.cursor_pos - 1);
                self.cursor_pos -= 1;
            }
            KeyCode::Left => self.cursor_pos = self.cursor_pos.saturating_sub(1),
            KeyCode::Delete if self.cursor_pos < self.input.len() => {
                self.input.remove(self.cursor_pos);
            }
            KeyCode::Home => self.cursor_pos = 0,
            KeyCode::Right => self.cursor_pos = (self.cursor_pos + 1).min(self.input.len()),
            _ => {}
        }
    }

    pub fn cursor_position(&self, area: Rect) -> (u16, u16) {
        let x = area.x + self.cursor_pos as u16;
        let y = area.y;
        (x, y)
    }

    pub fn value(&self) -> &str {
        &self.input
    }
}

impl Widget for ColorInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let input_display = if self.input.is_empty() {
            "#______".to_string()
        } else {
            self.input.clone()
        };

        // Write the input string
        buf.set_string(area.x, area.y, &input_display, Style::default());

        // Render cursor if focused
        if self.focused {
            let (x, y) = self.cursor_position(area);
            let cell = Buffer::cell_mut(buf, Position::new(x, y)).unwrap();
            cell.set_char('|');
            cell.set_style(Style::default().add_modifier(Modifier::RAPID_BLINK));
        }
    }
}
