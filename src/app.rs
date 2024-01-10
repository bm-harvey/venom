use crate::task;
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
        result.task_db.add_task("task 1", Prioity::High); 
        result.task_db.add_task("task 2", Prioity::Low); 
        result.task_db.add_task("task 3", Prioity::Medium); 
        result.task_db.add_task("task 4", Prioity::Low); 
        result.task_db.add_task("task 5", Prioity::Medium); 

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
