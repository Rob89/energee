use ratatui::{
    layout::{Alignment,Layout,Direction,Constraint},
    style::{Color, Style, Stylize},
    Frame,
    widgets::*,
};

use crate::app::{App, MeterPoint, GroupBy};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(80),
        Constraint::Percentage(10),
    ])
    .split(frame.size());

    let consumption_grouping = match app.group_by {
        GroupBy::HalfHour => "Half-hourly",
        GroupBy::Hour => "Hourly",
        GroupBy::Day => "Daily",
        GroupBy::Week => "Weekly",
    };

    let mp = &app.meters[app.selected_meter];
    let detail = match mp {
        MeterPoint::Gas(g) => format!("Gas - Serial Number: {}", g.serial),
        MeterPoint::Electric(e) => format!("Electricity - Serial Number: {}", e.serial),
    };
    frame.render_widget(
        Paragraph::new(format!(
            "Welcome to Energee. A TUI for smart meter data for no reason. \n\
        Meter ({} of {}): {}",

            app.selected_meter + 1,
            app.meters.len(),
            detail,
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

    let data = match mp {
        MeterPoint::Gas(g) => &g.comsumption_data,
        MeterPoint::Electric(e) => &e.comsumption_data,
    };

    let chunk_size: usize = 12;

    if let Some(consumption_data) = data {
        let reversed = consumption_data
            .results
            .iter()
            .rev()
            .collect::<Vec<_>>();
        let groups = reversed
            .iter()
            .map(|x| Bar::default().value((x.consumption * 1000.0) as u64))
            .collect::<Vec<_>>()
        .chunks(chunk_size)
        .enumerate()
        .map(|(idx, &ref x)| BarGroup::default().bars(x).label(reversed[idx * chunk_size].interval_start.format("%H:%M (%d/%m)").to_string().into()))
        .collect::<Vec<_>>();
        

        let mut bc = BarChart::default()
            .block(Block::default().title(format!("Consumption {}", consumption_grouping)).borders(Borders::ALL))
            .bar_width(1)
            .bar_gap(1)
            .group_gap(0)
            .bar_style(Style::new().gray())
            .value_style(Style::new().black().bold().on_light_yellow())
            .label_style(Style::new().white())
            .direction(Direction::Vertical);
        for group in groups {
            bc = bc.data(group)
        }        
        frame.render_widget(bc, layout[1]);
    } else if app.loading {
        frame.render_widget(
            Paragraph::new(format!("Loading..."))
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

    frame.render_widget(
        Paragraph::new(format!(
            "Press `Esc`, `Ctrl-C` or `q` to stop running. Move between meters with the arrow keys (left and right).\n\
        H(a)lf hourly, (H)ourly, (D)aily, (W)eekly."
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
