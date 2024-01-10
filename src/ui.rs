use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout, Span},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let mut rows = vec![Row::new(vec!["", "Title"])];

    app.task_db
        .tasks()
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let active_task = idx == app.selected_task_idx;
            let content = task.title();

            use crate::task::Prioity::*;
            let color = match task.priority() {
                Low => Color::Green,
                Medium => Color::Yellow,
                High => Color::Red,
            };

            let style = Style::default().fg(color);

            let selected_col = if active_task {
                String::from("*")
            } else {
                String::from(" ")
            };
            let selected_col = Span::styled(selected_col, style);

            let row = Row::new(vec![selected_col, Span::styled(content, style)]);

            row
        })
        .for_each(|row| rows.push(row));

    let title_constraint = app
        .task_db
        .tasks()
        .iter()
        .map(|task| task.title().len() as u16)
        .max()
        .unwrap_or(15)
        + 1;

    let main_table = Table::new(rows, vec![Constraint::Length(1), Constraint::Length(title_constraint)]).block(
        Block::default()
            .title("Tasks")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    let summary_block = Paragraph::new(app.task_db.task(app.selected_task_idx).unwrap().title())
        .block(
            Block::default()
                .title("Summary")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.size());

    f.render_widget(main_table, layout[0]);
    f.render_widget(summary_block, layout[1]);
}
