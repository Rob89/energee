use ratatui::{
    layout::{Alignment,Layout,Direction,Constraint},
    style::{Color, Style, Stylize},
    Frame,
    widgets::*,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(85),
        Constraint::Percentage(5),
    ])
    .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!(
            "Welcome to Energee. A TUI for smart meter data for no reason. \n\
        Meter ({} of {}): MPAN:{} Serial Number:{}",

            app.selected_meter + 1,
            app.meters.len(),
            app.meters[app.selected_meter].mpan,
            app.meters[app.selected_meter].serial
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

    if let Some(consumption_data) = &app.meters[app.selected_meter].comsumption_data {
        let data = consumption_data
            .results
            .iter()
            .map(|x| Bar::default().label(x.interval_start.format("%c").to_string().into()).value((x.consumption * 1000.0) as u64)).collect::<Vec<_>>();
        let bc = BarChart::default()
            .block(Block::default().title("Consumption").borders(Borders::ALL))
            .bar_width(1)
            .bar_gap(0)
            .group_gap(8)
            .bar_style(Style::new().yellow())
            .value_style(Style::new().red().bold())
            .label_style(Style::new().white())
            .direction(Direction::Horizontal)
            .data(BarGroup::default().bars(&data));
        frame.render_widget(bc, layout[1]);
    }

    frame.render_widget(
        Paragraph::new(format!(
            "Press `Esc`, `Ctrl-C` or `q` to stop running. Move between meters with the arrow keys (left and right)."
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        layout[2]
    );
}
