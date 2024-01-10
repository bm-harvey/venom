use crate::task::{self, Task};
use datetime::LocalDateTime;
use task::TaskDB;

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub task_db: TaskDB,
    pub selected_task_idx: usize,
}

use crate::task::Prioity;
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_example() -> Self {
        let mut result = Self::default();

        let date = datetime::LocalDate::ymd(2024, datetime::Month::February, 13).unwrap();
        let time = datetime::LocalTime::hm(12, 30).unwrap();
        let date_time = datetime::LocalDateTime::new(date, time);

        let example_task_1 = Task::builder()
            .with_title("Do Homework")
            .with_summary("Problems 1-7 on page 323 of Krane")
            .with_priority(Prioity::Medium)
            .with_due_date(Some(date))
            .build();

        let example_task_2 = Task::builder()
            .with_title("Call Doctor")
            .with_summary("(123) 456-7890")
            .with_priority(Prioity::Low)
            .build();

        let example_task_3 = Task::builder()
            .with_title("Write Paper")
            .with_summary("Section 4 requires major revision")
            .with_priority(Prioity::High)
            .build();

        result.task_db.add_task(example_task_1);
        result.task_db.add_task(example_task_2);
        result.task_db.add_task(example_task_3);

        result
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_task_idx(&mut self) {
        if self.task_db.is_empty() {
            self.selected_task_idx = 0;
            return;
        }

        let overflow = self.selected_task_idx + 1 == self.task_db.len();
        self.selected_task_idx = if overflow {
            0
        } else {
            self.selected_task_idx + 1
        }
    }

    pub fn decrement_task_idx(&mut self) {
        if self.task_db.is_empty() {
            self.selected_task_idx = 0;
            return;
        }

        let underflow = self.selected_task_idx == 0;
        self.selected_task_idx = if underflow {
            self.task_db.len() - 1
        } else {
            self.selected_task_idx - 1
        }
    }
}
