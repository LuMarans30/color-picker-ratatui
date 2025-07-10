use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

#[derive(Debug, Default, Clone)]
pub struct ColorInput {
    pub input: String,
    pub cursor_pos: usize,
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

    pub fn is_valid(&self) -> bool {
        self.input.len() == 6 && self.input.chars().all(|c| c.is_ascii_hexdigit())
    }
}
