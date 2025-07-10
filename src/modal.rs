use std::rc::Rc;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::{Color, Style, palette::material},
    widgets::{Block, BorderType, Borders, Clear, Widget},
};

use crate::{
    button::{Button, State},
    color_input::ColorInput,
};

#[derive(Debug)]
pub struct ColorPickerWidget {
    pub modal_state: bool,
    pub selected_color: Option<Color>,
    pub grid_index: (usize, usize),
    pub color_input: ColorInput,
    pub focus: Focus,
    pub colors: Vec<Color>,
    pub grid_dimensions: (usize, usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Focus {
    Grid,
    Input,
    Apply,
    Cancel,
}

impl Default for Focus {
    fn default() -> Self {
        Self::Grid
    }
}

impl ColorPickerWidget {
    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            Focus::Grid => Focus::Input,
            Focus::Input => Focus::Apply,
            Focus::Apply => Focus::Cancel,
            Focus::Cancel => Focus::Grid,
        };
    }

    pub fn focus_prev(&mut self) {
        self.focus = match self.focus {
            Focus::Grid => Focus::Cancel,
            Focus::Input => Focus::Grid,
            Focus::Apply => Focus::Input,
            Focus::Cancel => Focus::Apply,
        };
    }

    pub fn get_current_color(&self) -> Color {
        let (_, cols) = self.grid_dimensions;
        let idx = self.grid_index.0 * cols + self.grid_index.1;
        self.colors.get(idx).cloned().unwrap_or(Color::Black)
    }

    pub fn generate_colors() -> Vec<Color> {
        // Get all material design colors
        let hues = [
            &material::RED,
            &material::PINK,
            &material::PURPLE,
            &material::DEEP_PURPLE,
            &material::INDIGO,
            &material::BLUE,
            &material::LIGHT_BLUE,
            &material::CYAN,
            &material::TEAL,
            &material::GREEN,
            &material::LIGHT_GREEN,
            &material::LIME,
            &material::YELLOW,
            &material::AMBER,
            &material::ORANGE,
            &material::DEEP_ORANGE,
        ];

        // Get all accent levels (50 to 900)
        let accents = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900];

        let mut colors = Vec::new();

        // Create a distinct palette by selecting colors across hues and accents
        for &accent in &accents {
            for hue in &hues {
                let color = match accent {
                    50 => hue.c50,
                    100 => hue.c100,
                    200 => hue.c200,
                    300 => hue.c300,
                    400 => hue.c400,
                    500 => hue.c500,
                    600 => hue.c600,
                    700 => hue.c700,
                    800 => hue.c800,
                    900 => hue.c900,
                    _ => hue.c500, // fallback
                };
                colors.push(color);
            }
        }

        colors
    }
}

impl Default for ColorPickerWidget {
    fn default() -> Self {
        let colors = ColorPickerWidget::generate_colors();
        let grid_dimensions = (10, 16);

        Self {
            modal_state: false,
            selected_color: None,
            grid_index: (0, 0),
            color_input: ColorInput::default(),
            focus: Focus::default(),
            colors,
            grid_dimensions,
        }
    }
}

impl Widget for &ColorPickerWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.modal_state {
            return;
        }

        let area = modal_area(area, 50, 50);
        Clear.render(area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Color Picker")
            .style(Style::default().bg(Color::DarkGray));

        block.clone().render(area, buf);

        let inner_area = block.inner(area);
        let popup_layout = Layout::vertical([
            Constraint::Percentage(85),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(inner_area);

        let buttons_layout = Layout::horizontal([
            Constraint::Length(15),
            Constraint::Length(2),
            Constraint::Length(15),
        ])
        .flex(Flex::End)
        .split(popup_layout[2]);

        self.render_color_palette(popup_layout[0], buf);
        self.render_text_inputs(popup_layout[1], buf);
        self.render_modal_buttons(buttons_layout, buf);
    }
}

impl ColorPickerWidget {
    fn render_color_palette(&self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focus == Focus::Grid {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let grid_block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style);

        grid_block.clone().render(area, buf);
        let inner = grid_block.inner(area);

        let (rows, cols) = self.grid_dimensions;
        let total_colors = self.colors.len();

        let row_constraints = vec![Constraint::Ratio(1, rows as u32); rows];
        let grid_layout = Layout::vertical(row_constraints).split(inner);

        for row in 0..rows {
            let col_constraints = vec![Constraint::Ratio(1, cols as u32); cols];
            let row_layout = Layout::horizontal(col_constraints).split(grid_layout[row]);

            for col in 0..cols {
                let idx = row * cols + col;
                if idx >= total_colors {
                    continue;
                }

                let cell = row_layout[col];
                let is_selected = self.grid_index == (row, col);
                let color = self.colors[idx];

                // Fill cell with color
                for y in cell.top()..cell.bottom() {
                    for x in cell.left()..cell.right() {
                        let buf_cell = Buffer::cell_mut(buf, Position::new(x, y)).unwrap();
                        buf_cell.set_bg(color).set_fg(color);
                    }
                }

                // Draw selection border if selected
                if is_selected {
                    let selection_block = Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White));
                    selection_block.render(cell, buf);
                }
            }
        }
    }

    fn render_modal_buttons(&self, buttons: Rc<[Rect]>, buf: &mut Buffer) {
        let apply_focused = self.focus == Focus::Apply;
        let cancel_focused = self.focus == Focus::Cancel;

        Button::new("Apply")
            .state(if apply_focused {
                State::Selected
            } else {
                State::Normal
            })
            .focused(apply_focused) // Pass focus state
            .render(buttons[0], buf);

        Button::new("Cancel")
            .state(if cancel_focused {
                State::Selected
            } else {
                State::Normal
            })
            .focused(cancel_focused) // Pass focus state
            .render(buttons[2], buf);
    }

    fn render_text_inputs(&self, area: Rect, buf: &mut Buffer) {
        // Use cyan border when focused
        let border_style = if self.focus == Focus::Input {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title("HEX Color")
            .border_style(border_style);

        input_block.render(area, buf);

        // Only use the middle row for input
        let input_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width - 2,
            height: 1,
        };

        let mut input = self.color_input.clone();
        input.focused = self.focus == Focus::Input;
        input.render(input_area, buf);
    }

    pub fn color_to_hex(color: Color) -> String {
        match color {
            Color::Rgb(r, g, b) => format!("{r:02X}{g:02X}{b:02X}"),
            _ => String::new(),
        }
    }
}

fn modal_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_width = (area.width * percent_x) / 100;
    let popup_height = (area.height * percent_y) / 100;
    let vertical_margin = (area.height - popup_height) / 2;
    let horizontal_margin = (area.width - popup_width) / 2;

    Rect::new(
        area.x + horizontal_margin,
        area.y + vertical_margin,
        popup_width,
        popup_height,
    )
}
