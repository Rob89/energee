
use ratatui::{
    layout::{Alignment,Layout,Direction,Constraint},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(90),
        Constraint::Percentage(10),
    ])
    .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!(
            "Welcome to Energee. A TUI for smart data for no reason. \n\
        \n\
        Electricity: {} (serial: {})\n\
        Gas: {} (serial: {})",
            app.electricity.mpan,
            app.electricity.serial,
            app.gas.mpan,
            app.gas.serial,
        ))
        .block(
            Block::default()
                .title("Energee")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        layout[0]
    );
    frame.render_widget(
        Paragraph::new(format!(
            "Press `Esc`, `Ctrl-C` or `q` to stop running."
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        layout[1]
    );
}
