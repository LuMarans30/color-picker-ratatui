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
    Quit,
    Ignore,
}

pub fn update(model: &mut Model, message: Message) -> Result<bool> {
    match message {
        Message::KeyPress(key) if key.kind == KeyEventKind::Press => {
            if model.color_picker.modal_state {
                handle_modal_keys(model, key)
            } else {
                handle_global_keys(model, key)
            }
        }
        Message::UpdateColorFromGrid => {
            if let Some(color) = model.color_picker.selected_color()
                && let Some(hex) = ColorPickerWidget::color_to_hex(color)
            {
                model.color_picker.color_input.input = hex;
                model.color_picker.color_input.cursor_pos =
                    model.color_picker.color_input.input.len();
            }
            Ok(true)
        }
        Message::ApplyColor => {
            model.color_picker.modal_state = false;
            Ok(true)
        }
        Message::CancelColorSelection => {
            model.color_picker.modal_state = false;
            Ok(true)
        }
        Message::ToggleModal => {
            model.color_picker.modal_state = !model.color_picker.modal_state;
            if model.color_picker.modal_state
                && let Some(color) = model.color_picker.selected_color()
                && let Some(hex) = ColorPickerWidget::color_to_hex(color)
            {
                model.color_picker.color_input.input = hex;
                model.color_picker.color_input.cursor_pos =
                    model.color_picker.color_input.input.len();
            }
            Ok(true)
        }
        Message::Quit => Ok(false),
        Message::Ignore => Ok(true),
        _ => Ok(true),
    }
}

fn handle_modal_keys(model: &mut Model, key: KeyEvent) -> Result<bool> {
    if model.color_picker.focus == Focus::Grid {
        let (mut row, mut col) = model.color_picker.grid_index;
        let (rows, cols) = model.color_picker.grid_dimensions;
        let max_row = rows.saturating_sub(1);
        let max_col = cols.saturating_sub(1);

        match key.code {
            KeyCode::Up => row = row.saturating_sub(1),
            KeyCode::Down => row = (row + 1).min(max_row),
            KeyCode::Left => col = col.saturating_sub(1),
            KeyCode::Right => col = (col + 1).min(max_col),
            _ => return Ok(true),
        }

        model.color_picker.grid_index = (row, col);
        return update(model, Message::UpdateColorFromGrid);
    }

    match key.code {
        KeyCode::Esc => update(model, Message::CancelColorSelection),
        KeyCode::Tab => {
            model.color_picker.focus_next();
            Ok(true)
        }
        KeyCode::BackTab => {
            model.color_picker.focus_prev();
            Ok(true)
        }
        KeyCode::Enter => match model.color_picker.focus {
            Focus::Apply => update(model, Message::ApplyColor),
            Focus::Cancel => update(model, Message::CancelColorSelection),
            _ => Ok(true),
        },
        _ => {
            if let Focus::Input = model.color_picker.focus {
                model.color_picker.color_input.handle_key_event(key);
            }
            Ok(true)
        }
    }
}

fn handle_global_keys(model: &mut Model, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') => Ok(false),
        KeyCode::Char('p') => update(model, Message::ToggleModal),
        _ => Ok(true),
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
