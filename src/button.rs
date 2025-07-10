use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Widget},
};

#[derive(Debug, Clone)]
pub struct Button<'a> {
    label: Line<'a>,
    pub state: State,
    focused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Selected,
}

impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Self {
            label: label.into(),
            state: State::Normal,
            focused: false,
        }
    }

    pub fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }
}

impl Widget for Button<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (bg, fg) = match self.state {
            State::Selected => (Color::Blue, Color::White),
            State::Normal => (Color::DarkGray, Color::Gray),
        };

        // Use cyan border when focused
        let border_style = Style::default().fg(if self.focused { Color::Cyan } else { fg });

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .style(Style::default().bg(bg).fg(fg));

        block.render(area, buf);

        let label_width = self.label.width() as u16;
        let x = area.x + (area.width.saturating_sub(label_width)) / 2;
        let y = area.y + area.height / 2;

        buf.set_line(x, y, &self.label, area.width);
    }
}
