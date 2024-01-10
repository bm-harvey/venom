use std::vec;

use crate::app::App;
use crate::task::Priority;
//use datetime::DatePiece;
use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout, Line, Span, Text},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Row, Table},
};

pub fn render(app: &mut App, f: &mut Frame) {
    let header_style = Style::default().fg(Color::default()).underlined();

    let due_date_col_name = "Due Date".to_string();
    let due_date_col_name = Span::styled(due_date_col_name, header_style);

    let due_time_col_name = "Due Time".to_string();
    let due_time_col_name = Span::styled(due_time_col_name, header_style);

    let title_col_name = "Title".to_string();
    let title_col_name = Span::styled(title_col_name, header_style);

    let label_col_name = "Label".to_string();
    let label_col_name = Span::styled(label_col_name, header_style);

    let mut rows = vec![Row::new(vec![
        Span::default(),
        title_col_name,
        label_col_name.clone(),
        due_date_col_name.clone(),
        due_time_col_name.clone(),
    ])];

    let mut date_constraint = due_date_col_name.width() as u16;
    let mut time_constraint = due_time_col_name.width() as u16;
    app.task_db
        .tasks()
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let active_task = idx == app.selected_task_idx;

            let color = match task.priority() {
                Priority::None => Color::default(),
                Priority::Low => Color::Green,
                Priority::Medium => Color::Yellow,
                Priority::High => Color::Red,
            };

            let mut label_style = Style::default();
            if task.label().is_some() {
                label_style = label_style.fg(task.label().as_ref().unwrap().color());
            }
            let label_col = match task.label().as_ref() {
                None => "".to_string(),
                Some(label) => label.short_name().iter().collect::<String>(),
            };
            let label_col = Span::styled(label_col, label_style);

            let style = Style::default().fg(color);
            let content_col = task.title();
            let content_col = Span::styled(content_col, style);

            let selected_col = if active_task {
                String::from("*")
            } else {
                String::from(" ")
            };
            let selected_col = Span::styled(selected_col, style);

            let due_date_col = task.date_string();
            let due_time_col = task.time_string();
            date_constraint = std::cmp::max(date_constraint, due_date_col.len() as u16);
            time_constraint = std::cmp::max(time_constraint, due_time_col.len() as u16);
            let due_date_col = Span::styled(due_date_col, style);
            let due_time_col = Span::styled(due_time_col, style);

            let row = Row::new(vec![
                selected_col,
                content_col,
                label_col,
                due_date_col,
                due_time_col,
            ]);

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

    let main_table = Table::new(
        rows,
        Constraint::from_lengths([
            1,
            title_constraint + 1,
            5,
            date_constraint + 1,
            time_constraint + 1,
        ]),
    )
    .block(
        Block::default()
            .title("Tasks")
            .padding(Padding::new(1, 1, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    let active_task = app.task_db.task(app.selected_task_idx).unwrap();
    let active_priority = active_task.priority();

    let mut summary_text = vec![
        Line::raw(format!("Title   : {}", active_task.title())),
        Line::raw(format!(
            "Due Date: {} {}",
            active_task.date_string(),
            active_task.time_string()
        )),
        Line::from(vec![Span::raw("Priority: "), active_priority.as_span()]),
    ];

    summary_text.push(Line::from(vec![
        Span::raw("Label   : "),
        match active_task.label() {
            None => Span::default(),
            Some(label) => label.as_span(),
        },
    ]));

    summary_text.push(Line::default());
    summary_text.push(Line::raw("Notes   :"));
    active_task
        .notes()
        .lines()
        .map(Line::raw)
        .for_each(|line| summary_text.push(line));

    let summary_block = Paragraph::new(summary_text).block(
        Block::default()
            .title("Summary")
            .padding(Padding::new(1, 1, 1, 1))
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
