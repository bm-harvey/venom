use crate::edit_task_popup::EditTaskPopup;
use crate::task::{self, Task, TaskLabel};
use ratatui::style::Color;
use std::cell::RefCell;
use std::rc::Rc;
use task::TaskDB;

/// The global app state
#[derive(Default)]
pub struct Venom {
    should_quit: bool,
    task_db: TaskDB,
    labels: Vec<Rc<TaskLabel>>,
    selected_task_idx: usize,
    focus: VenomFocus,
}

/// The Current Focus of a Venom Application
#[derive(Default, Clone)]
pub enum VenomFocus {
    #[default]
    MainView,
    EditPopup(Rc<RefCell<EditTaskPopup>>),
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialOrd,
    PartialEq,
    Eq,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
    strum::EnumIs,
)]
pub enum EditableTaskProperty {
    #[default]
    Title,
    Notes,
    DueDate,
}

use crate::task::Priority;
impl Venom {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    pub fn task_db(&self) -> &TaskDB {
        &self.task_db
    }
    pub fn selected_task_idx(&self) -> usize {
        self.selected_task_idx
    }
    pub fn selected_task(&self) -> Rc<RefCell<Task>> {
        self.task_db.tasks()[self.selected_task_idx].clone()
    }

    pub fn new_example() -> Self {
        let mut result = Self::default();

        let personal_label = Rc::new(TaskLabel::new("Personal", "PSNL", Color::Blue));
        let research_label = Rc::new(TaskLabel::new("Research", "RSCH", Color::LightMagenta));
        let class_label = Rc::new(TaskLabel::new("Class", "CLSS", Color::Rgb(200, 120, 0)));

        let date = datetime::LocalDate::ymd(2024, datetime::Month::February, 13).unwrap();
        let time = datetime::LocalTime::hm(12, 30).unwrap();

        let example_task_1 = Task::builder()
            .with_title("Do Homework")
            .with_notes("Problems 1-7 on page 323 of Krane\nProblems 1-2 on page 324")
            .with_priority(Priority::Medium)
            .with_due_date(Some(date))
            .with_due_time(Some(time))
            .with_label(Some(class_label.clone()))
            .build();

        let date = datetime::LocalDate::ymd(2024, datetime::Month::April, 3).unwrap();
        let example_task_2 = Task::builder()
            .with_title("Call Doctor")
            .with_notes("(123) 456-7890")
            .with_priority(Priority::Low)
            .with_due_date(Some(date))
            .with_label(Some(personal_label.clone()))
            .build();

        let example_task_3 = Task::builder()
            .with_title("Write Paper")
            .with_notes("Section 4 requires major revision")
            .with_priority(Priority::High)
            .with_label(Some(research_label.clone()))
            .build();

        let example_task_4 = Task::builder()
            .with_title("Write Other Paper")
            .with_notes("Oh yeah I have another paper to write")
            .with_label(Some(research_label.clone()))
            .build();

        result.labels.push(personal_label);
        result.labels.push(research_label);
        result.labels.push(class_label);

        result.task_db.add_task(example_task_1);
        result.task_db.add_task(example_task_2);
        result.task_db.add_task(example_task_3);
        result.task_db.add_task(example_task_4);

        result
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn focus(&self) -> &VenomFocus {
        &self.focus
    }

    pub fn set_mode(&mut self, mode: VenomFocus) {
        self.focus = mode;
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

    pub fn add_task(&mut self) {
        let task = Task::builder().with_title("Added Task").build();
        self.task_db.add_task(task);
        self.selected_task_idx = self.task_db.len() - 1;
        self.edit_task();
    }

    pub fn edit_task(&mut self) {
        let popup = Rc::new(RefCell::new(EditTaskPopup::default()));
        let property = popup.borrow().property();
        popup
            .borrow_mut()
            .load_text(&self.selected_task().borrow().text_to_edit(property));
        self.focus = VenomFocus::EditPopup(popup);
    }
}
