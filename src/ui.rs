use std::vec;

use crate::app::{self, App, AppMode, EditTaskFocus};

use crate::task::Priority;
use ratatui::widgets::Clear;
//use datetime::DatePiece;
use ratatui::{
    prelude::*,
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Row, Table},
};
use strum::IntoEnumIterator;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(f.size());

    let main_table = main_table(app);
    let summary_block = summary_block(app);

    f.render_widget(main_table, layout[0]);
    f.render_widget(summary_block, layout[1]);

    if app.mode() == AppMode::EditPopup {
        render_edit_task_popup(app, f);
    }
}

fn render_edit_task_popup(app: &mut App, frame: &mut Frame) {
    let area = centered_rect(frame.size(), 60, 50);

    let popup = app.edit_task_popup_mut();
    if let Some(popup) = popup {
        let active_color = Color::default();
        let inactive_color = Color::DarkGray;
        let highlight_color = Color::Rgb(0, 50, 200);
        let inactive_style = Style::default().fg(inactive_color);
        let active_style = Style::default().fg(active_color);
        let highlight_style = Style::default().fg(highlight_color).bold().italic();

        let mut field_style = active_style;
        let mut edit_style = inactive_style;

        if popup.focus() == EditTaskFocus::Edit {
            std::mem::swap(&mut field_style, &mut edit_style);
        }

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);

        let field_block = Block::default()
            .title(" Field ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .set_style(field_style);

        let edit_block = Block::default()
            .title(format!(" Editing: {} ", popup.property()))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .set_style(edit_style);

        //let edit_paragraph = match popup.property() {
        //app::EditableTaskProperty::Title => {
        //Paragraph::new(Line::styled(task.title(), edit_style))
        //},
        //app::EditableTaskProperty::Notes => {
        //Paragraph::new(Line::styled(task.notes(), edit_style))
        //},
        //app::EditableTaskProperty::DueDate => {
        //Paragraph::new(Line::styled(format!("{} {}", task.date_string(), task.time_string()), edit_style))
        //},
        ////_ => Paragraph::default(),
        //}
        //.block(edit_block);
        //

        let property = popup.property();
        let edit_paragraph = popup.text_editor_widget();

        let mut field_rows = vec![];
        for field in app::EditableTaskProperty::iter() {
            let style = if property == field {
                highlight_style
            } else {
                field_style
            };
            field_rows.push(Row::new(vec![Span::styled(field.to_string(), style)]));
        }

        let field_table =
            Table::new(field_rows, vec![Constraint::Percentage(100)]).block(field_block);

        frame.render_widget(Clear, area);
        frame.render_widget(field_table, layout[0]);
        frame.render_widget(edit_paragraph, layout[1]);
    }
}

fn summary_block(app: &App) -> Paragraph {
    let active_task = app.task_db().task(app.selected_task_idx()).unwrap();
    let prio = active_task.priority();
    let (color, word) = prio.formatting();

    let mut summary_text = vec![
        Line::raw(format!("Title   : {}", active_task.title())),
        Line::raw(format!(
            "Due Date: {} {}",
            active_task.date_string(),
            active_task.time_string()
        )),
        Line::from(vec![
            Span::raw("Priority: "),
            Span::styled(word, Style::default().fg(color)),
        ]),
    ];

    let label = active_task.label();
    let label = label.clone();
    summary_text.push(Line::from(vec![
        Span::raw("Label   : "),
        match label {
            None => Span::default(),
            Some(_) => Span::default(),
            //Some(label) => label.as_span(),
        },
    ]));

    summary_text.push(Line::default());
    summary_text.push(Line::raw("Notes   :"));
    active_task
        .notes()
        .lines()
        .map(|line| line.to_string())
        .map(Line::raw)
        .for_each(|line| summary_text.push(line));

    Paragraph::new(summary_text).block(
        Block::default()
            .title(" Summary ")
            .padding(Padding::new(1, 1, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick),
    )
}

fn main_table<'a>(app: &'a App) -> Table<'a> {
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
    app.task_db()
        .tasks()
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            let active_task = idx == app.selected_task_idx();

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
        .task_db()
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
            .title(" Tasks ")
            .padding(Padding::new(1, 1, 1, 1))
            .borders(Borders::ALL)
            .border_type(BorderType::Thick),
    );
    main_table
}

/// # Usage
///
/// ```rust
/// let rect = centered_rect(f.size(), 50, 50);
/// ```
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
