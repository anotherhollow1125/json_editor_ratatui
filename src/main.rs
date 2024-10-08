use std::io;

use anyhow::Result;
use app::{App, CurrentScreen, CurrentlyEditing};
use ratatui::crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::{Backend, CrosstermBackend};
use ratatui::Terminal;

mod app;
mod ui;
use ui::ui;

fn main() -> Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create aoo and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // show result
    match res {
        Ok(true) => app.print_json()?,
        Err(e) => println!("{e:?}"),
        _ => {}
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
    let do_print = loop {
        terminal.draw(|f| ui(f, app))?;

        let Event::Key(KeyEvent {
            kind: event::KeyEventKind::Press,
            code,
            ..
        }) = event::read()?
        else {
            continue;
        };

        match app.current_screen {
            CurrentScreen::Main => main_screen_handler(app, code),
            CurrentScreen::Editing(_) => editing_screen_handler(app, code),
            CurrentScreen::Exiting => {
                if let Some(do_print) = exiting_screen_handler(code) {
                    break do_print;
                }
            }
        }
    };

    Ok(do_print)
}

fn main_screen_handler(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('e') => {
            app.current_screen = CurrentScreen::Editing(CurrentlyEditing::Key);
        }
        KeyCode::Char('q') => {
            app.current_screen = CurrentScreen::Exiting;
        }
        _ => {}
    }
}

fn exiting_screen_handler(key_code: KeyCode) -> Option<bool> {
    match key_code {
        KeyCode::Char('y') => Some(true),
        KeyCode::Char('n') | KeyCode::Char('q') => Some(false),
        _ => None,
    }
}

fn editing_screen_handler(app: &mut App, key_code: KeyCode) {
    use CurrentScreen::*;
    use CurrentlyEditing::*;

    match key_code {
        KeyCode::Esc => {
            app.current_screen = Main;
        }
        KeyCode::Tab => {
            app.toggle_editing();
        }
        key_code => match app.current_screen {
            Editing(Key) => editing_key_handler(app, key_code),
            Editing(Value) => editing_value_handler(app, key_code),
            _ => {}
        },
    }
}

fn editing_key_handler(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => {
            app.current_screen = CurrentScreen::Editing(CurrentlyEditing::Value);
        }
        KeyCode::Backspace => {
            app.key_input.pop();
        }
        KeyCode::Char(char) => {
            app.key_input.push(char);
        }
        _ => {}
    }
}

fn editing_value_handler(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Enter => {
            app.save_key_value();
            app.current_screen = CurrentScreen::Main;
        }
        KeyCode::Backspace => {
            app.value_input.pop();
        }
        KeyCode::Char(char) => {
            app.value_input.push(char);
        }
        _ => {}
    }
}
