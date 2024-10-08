use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    main_screen(frame, app);
}

fn main_screen(frame: &mut Frame, app: &App) {
    // Create Chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Title
    frame.render_widget(title(), chunks[0]);

    // Json KV Pairs List
    frame.render_widget(kv_list(app), chunks[1]);

    // Footers
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    frame.render_widget(mode_footer(app), footer_chunks[0]);
    frame.render_widget(key_notes_footer(app), footer_chunks[1]);

    // Popups
    match app.current_screen {
        CurrentScreen::Editing(_) => draw_editing_popup(frame, app),
        CurrentScreen::Exiting => draw_exiting_popup(frame),
        _ => {}
    }
}

fn title() -> Paragraph<'static> {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block)
}

fn kv_list(app: &App) -> List {
    let list_items = app
        .pairs
        .iter()
        .map(|(key, value)| {
            let content = format!("{: <25} : {}", key, value);
            ListItem::new(Line::from(Span::styled(
                content,
                Style::default().fg(Color::Yellow),
            )))
        })
        .collect::<Vec<ListItem>>();
    List::new(list_items)
}

fn mode_footer(app: &App) -> Paragraph<'static> {
    Paragraph::new(Line::from(vec![
        mode_name_span(app),
        Span::styled(" | ", Style::default().fg(Color::White)),
        edit_target_span(app),
    ]))
    .block(Block::default().borders(Borders::ALL))
}

fn mode_name_span(app: &App) -> Span<'static> {
    match app.current_screen {
        CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
        CurrentScreen::Editing(_) => {
            Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
        }
        CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
    }
}

fn edit_target_span(app: &App) -> Span<'static> {
    match app.current_screen {
        CurrentScreen::Editing(CurrentlyEditing::Key) => {
            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
        }
        CurrentScreen::Editing(CurrentlyEditing::Value) => {
            Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
        }
        _ => Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray)),
    }
}

fn key_notes_footer(app: &App) -> Paragraph<'static> {
    let content = match app.current_screen {
        CurrentScreen::Main | CurrentScreen::Exiting => "(q) to quit / (e) to make new pair",
        CurrentScreen::Editing(_) => "(ESC) to cancel / (Tab) to switch boxes / enter to complete",
    };
    let current_keys_hint = Span::styled(content, Style::default().fg(Color::Red));

    Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL))
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn draw_editing_popup(frame: &mut Frame, app: &App) {
    let popup_block = editing_popup_block();
    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let (key_text, value_text) = key_value_blocks(app);
    frame.render_widget(key_text, popup_chunks[0]);
    frame.render_widget(value_text, popup_chunks[1]);
}

fn editing_popup_block() -> Block<'static> {
    Block::default()
        .title("Enter a new key-value pair")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray))
}

fn key_value_blocks(app: &App) -> (Paragraph<'static>, Paragraph<'static>) {
    let mut key_block = Block::default().title("Key").borders(Borders::ALL);
    let mut value_block = Block::default().title("Value").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    match app.current_screen {
        CurrentScreen::Editing(CurrentlyEditing::Key) => key_block = key_block.style(active_style),
        CurrentScreen::Editing(CurrentlyEditing::Value) => {
            value_block = value_block.style(active_style)
        }
        _ => {} // Unreachable
    }

    let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
    let value_text = Paragraph::new(app.value_input.clone()).block(value_block);

    (key_text, value_text)
}

fn draw_exiting_popup(frame: &mut Frame) {
    // frame.render_widget(Clear, frame.area());
    let popup_block = Block::default()
        .title("Y/N")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let exit_test = Text::styled(
        "Would you like to output the buffer as json? (y/n)",
        Style::default().fg(Color::Red),
    );
    let exit_paragraph = Paragraph::new(exit_test)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 25, frame.area());
    frame.render_widget(exit_paragraph, area);
}
