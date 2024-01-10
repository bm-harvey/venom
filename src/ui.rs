use crate::app::App;
use crate::task::Prioity;
use datetime::DatePiece;
use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout, Span},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Row, Table},
};

pub fn render(app: &mut App, f: &mut Frame) {
    let header_style = Style::default().fg(Color::LightMagenta);

    let due_date_col_name = "Due Date".to_string();
    let due_date_col_name = Span::styled(due_date_col_name, header_style);

    let title_col_name = "Title".to_string();
    let title_col_name = Span::styled(title_col_name, header_style);

    let mut rows = vec![Row::new(vec![Span::default(), title_col_name, due_date_col_name.clone()])];

    let mut date_constraint = due_date_col_name.width() as u16;
    app.task_db
        .tasks()
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let active_task = idx == app.selected_task_idx;

            let color = match task.priority() {
                Prioity::None => Color::default(),
                Prioity::Low => Color::Green,
                Prioity::Medium => Color::Yellow,
                Prioity::High => Color::Red,
            };

            let style = Style::default().fg(color);
            let content_col = task.title();
            let content_col = Span::styled(content_col, style);

            let selected_col = if active_task {
                String::from("*")
            } else {
                String::from(" ")
            };
            let selected_col = Span::styled(selected_col, style);

            let due_date_col = match task.due_date() {
                None => "".to_string(),
                Some(dt) => {
                    let result = match dt.month() {
                        datetime::Month::January => "Jan",
                        datetime::Month::February => "Feb",
                        datetime::Month::March => "Mar",
                        _ => "Other",
                    }
                    .to_string();

                    date_constraint = std::cmp::max(result.len() as u16, date_constraint);
                    result
                }
            };
            let due_date_col = Span::styled(due_date_col, style);

            let row = Row::new(vec![selected_col, content_col, due_date_col]);

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
        Constraint::from_lengths([1, title_constraint, date_constraint]),
    )
    .block(
        Block::default()
            .title("Tasks")
            .padding(Padding::new(1, 1, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    );

    let summary_block = Paragraph::new(app.task_db.task(app.selected_task_idx).unwrap().summary())
        .block(
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
