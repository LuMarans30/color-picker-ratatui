use color_eyre::Result;
use crossterm::event;
use ratatui::{
    Terminal,
    crossterm::event::{KeyCode, KeyEvent, KeyEventKind},
    prelude::CrosstermBackend,
};
use std::io::Stdout;

use crate::modal::{ColorPickerWidget, Focus};

mod button;
mod color_input;
mod modal;
mod util {
    pub mod styles;
}

#[derive(Debug, Default)]
pub struct Model {
    color_picker: ColorPickerWidget,
}

#[derive(Debug)]
pub enum Message {
    KeyPress(KeyEvent),
    ToggleModal,
    ApplyColor,
    UpdateColorFromGrid,
    CancelColorSelection,
    FocusNext,
    FocusPrev,
    Quit,
    Ignore,
}

// Centralized key mapping
struct KeyHandler;

impl KeyHandler {
    fn handle_global_keys(key: KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q' | 'Q') => Some(Message::Quit),
            KeyCode::Char('p' | 'P') => Some(Message::ToggleModal),
            _ => None,
        }
    }

    fn handle_modal_navigation(model: &mut Model, key: KeyEvent) -> Option<Message> {
        if model.color_picker.focus != Focus::Grid {
            return None;
        }

        match key.code {
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                Self::update_grid_position(model, key.code);
                Some(Message::UpdateColorFromGrid)
            }
            _ => None,
        }
    }

    fn handle_modal_actions(model: &Model, key: KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Tab => Some(Message::FocusNext),
            KeyCode::BackTab => Some(Message::FocusPrev),
            KeyCode::Enter => match model.color_picker.focus {
                Focus::Apply => Some(Message::ApplyColor),
                Focus::Cancel => Some(Message::CancelColorSelection),
                _ => None,
            },
            KeyCode::Esc => Some(Message::CancelColorSelection),
            _ => None,
        }
    }

    fn handle_input_keys(model: &mut Model, key: KeyEvent) -> bool {
        if model.color_picker.focus == Focus::Input {
            model.color_picker.color_input.handle_key_event(key);
            true
        } else {
            false
        }
    }

    fn update_grid_position(model: &mut Model, key_code: KeyCode) {
        let (mut row, mut col) = model.color_picker.grid_index;
        let (rows, cols) = model.color_picker.grid_dimensions;
        let max_row = rows.saturating_sub(1);
        let max_col = cols.saturating_sub(1);

        match key_code {
            KeyCode::Up => row = row.saturating_sub(1),
            KeyCode::Down => row = (row + 1).min(max_row),
            KeyCode::Left => col = col.saturating_sub(1),
            KeyCode::Right => col = (col + 1).min(max_col),
            _ => unreachable!(),
        }

        model.color_picker.grid_index = (row, col);
    }
}

pub fn update(model: &mut Model, message: Message) -> Result<bool> {
    match message {
        Message::KeyPress(key) if key.kind == KeyEventKind::Press => handle_key_press(model, key),
        Message::UpdateColorFromGrid => {
            update_color_from_grid(model);
            Ok(true)
        }
        Message::ApplyColor => Ok(false),
        Message::CancelColorSelection => Ok(false),
        Message::ToggleModal => {
            toggle_modal(model);
            Ok(true)
        }
        Message::FocusNext => {
            model.color_picker.focus_next();
            Ok(true)
        }
        Message::FocusPrev => {
            model.color_picker.focus_prev();
            Ok(true)
        }
        Message::Quit => Ok(false),
        Message::Ignore => Ok(true),
        _ => Ok(true),
    }
}

fn handle_key_press(model: &mut Model, key: KeyEvent) -> Result<bool> {
    if let Some(message) = KeyHandler::handle_global_keys(key) {
        return update(model, message);
    }

    if model.color_picker.modal_state {
        if let Some(message) = KeyHandler::handle_modal_navigation(model, key) {
            return update(model, message);
        }

        if let Some(message) = KeyHandler::handle_modal_actions(model, key) {
            return update(model, message);
        }

        if KeyHandler::handle_input_keys(model, key) {
            return Ok(true);
        }
    }

    Ok(true)
}

fn update_color_from_grid(model: &mut Model) {
    if let Some(color) = model.color_picker.selected_color()
        && let Some(hex) = ColorPickerWidget::color_to_hex(color)
    {
        model.color_picker.color_input.input = hex.clone();
        model.color_picker.color_input.cursor_pos = hex.len();
    }
}

fn toggle_modal(model: &mut Model) {
    model.color_picker.modal_state = !model.color_picker.modal_state;

    if model.color_picker.modal_state {
        update_color_from_grid(model);
    }
}

pub fn handle_event() -> Result<Message> {
    match event::read()? {
        event::Event::Key(key) => Ok(Message::KeyPress(key)),
        event::Event::Resize(..) => Ok(Message::Ignore),
        _ => Ok(Message::Quit),
    }
}

pub fn view(model: &Model, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    terminal
        .draw(|frame| {
            frame.render_widget(&model.color_picker, frame.area());
        })
        .expect("Couldn't draw the UI");
}

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let mut model = Model::default();

    let mut running = true;
    while running {
        view(&model, &mut terminal);

        let message = handle_event()?;
        running = update(&mut model, message)?;
    }

    ratatui::restore();
    Ok(())
}
