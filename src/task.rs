use std::rc::Rc;

use datetime::{DatePiece, LocalDate, LocalTime, Month, TimePiece};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};

#[derive(Debug, Default)]
pub struct TaskDB {
    tasks: Vec<Task>,
}

impl TaskDB {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn add_default(&mut self) -> &mut Self {
        self.tasks.push(Task::default());
        self
    }

    pub fn task(&self, idx: usize) -> Option<&Task> {
        self.tasks.get(idx)
    }

    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.tasks.push(task);
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct Task {
    title: String,
    priority: Priority,
    notes: String,
    due_date: Option<LocalDate>,
    due_time: Option<LocalTime>,
    label: Option<Rc<TaskLabel>>,
}

impl Task {
    pub fn new(title: &str, priority: Priority) -> Self {
        Self {
            title: title.into(),
            priority,
            notes: "".to_string(),
            due_date: None,
            due_time: None,
            label: None,
        }
    }

    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn label(&self) -> &Option<Rc<TaskLabel>> {
        &self.label
    }

    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn due_date(&self) -> Option<LocalDate> {
        self.due_date
    }
    pub fn due_time(&self) -> Option<LocalTime> {
        self.due_time
    }

    pub fn date_string(&self) -> String {
        match self.due_date {
            None => "".to_string(),
            Some(date) => {
                let year_string = date.year().to_string();
                let month_string = match date.month() {
                    Month::January => "Jan",
                    Month::February => "Feb",
                    Month::March => "Mar",
                    Month::April => "Apr",
                    Month::May => "May",
                    Month::June => "Jun",
                    Month::July => "Jul",
                    Month::August => "Aug",
                    Month::September => "Sep",
                    Month::October => "Oct",
                    Month::November => "Nov",
                    Month::December => "Dec",
                }
                .to_string();

                let day_string = date.day().to_string();

                format!("{:2} {} {}", day_string, month_string, year_string)
            }
        }
    }
    pub fn time_string(&self) -> String {
        match self.due_time {
            None => "".to_string(),
            Some(time) => {
                let hour_string = time.hour().to_string();
                let min_string = time.minute().to_string();

                format!("{hour_string}:{min_string}")
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Priority {
    #[default]
    None,
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_text(&self) -> Text {
        let (color, word) = self.formatting();
        Text::styled(word, Style::default().fg(color))
    }
    pub fn as_span(&self) -> Span {
        let (color, word) = self.formatting();
        Span::styled(word, Style::default().fg(color))
    }
    pub fn as_line(&self) -> Line {
        let (color, word) = self.formatting();
        Line::styled(word, Style::default().fg(color))
    }

    fn formatting(&self) -> (Color, String) {
        match self {
            Self::None => (Color::default(), "".to_string()),
            Self::Low => (Color::Green, "Low".to_string()),
            Self::Medium => (Color::Yellow, "Medium".to_string()),
            Self::High => (Color::Red, "High".to_string()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TaskBuilder {
    task: Task,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Task {
        self.task
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.task.title = title.to_string();
        self
    }

    pub fn with_notes(mut self, notes: &str) -> Self {
        self.task.notes = notes.to_string();
        self
    }

    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.task.priority = priority;
        self
    }

    pub fn with_due_time(mut self, time: Option<LocalTime>) -> Self {
        self.task.due_time = time;
        self
    }

    pub fn with_due_date(mut self, date: Option<LocalDate>) -> Self {
        self.task.due_date = date;
        self
    }

    pub fn with_label(mut self, label: Option<Rc<TaskLabel>>) -> Self {
        self.task.label = label;
        self
    }
}

#[derive(Default, Debug, Clone)]
pub struct TaskLabel {
    color: Color,
    short_name: [char; Self::LABEL_LEN],
    long_name: String,
}

impl TaskLabel {
    pub const LABEL_LEN: usize = 4;

    pub fn new(long_name: &str, short_name: &str, color: Color) -> Self {
        let long_name = long_name.to_string();

        let short_name = {
            let mut result = [' '; Self::LABEL_LEN];
            for (idx, val) in short_name.chars().enumerate() {
                if idx >= Self::LABEL_LEN {
                    break;
                }
                result[idx] = val;
            }
            result
        };

        Self {
            short_name,
            long_name,
            color,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
    pub fn long_name(&self) -> &str {
        &self.long_name
    }
    pub fn short_name(&self) -> &[char] {
        &self.short_name
    }
    pub fn short_name_string(&self) -> String {
        self.short_name().iter().collect()
    }
    pub fn as_span(&self) -> Span {
        Span::styled(
            format!("{} ({})", self.long_name(), self.short_name_string()),
            Style::default().fg(self.color),
        )
    }
}
