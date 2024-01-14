use crate::edit_task_popup::EditTaskPopup;
use crate::task::{self, Task, TaskLabel, RGB};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use chrono::{Local, NaiveDateTime};
use std::cell::RefCell;
use std::rc::Rc;
use strum;
use task::TaskDB;

/// The global app state
#[derive(Default)]
pub struct Venom {
    should_quit: bool,
    task_db: TaskDB,
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
    Label,
}

use crate::task::Priority;
impl Venom {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.read_from_file();
        app
    }

    pub fn save_file(&self) {
        let write_path = self.save_dir_path();
        if !write_path.is_dir() {
            std::fs::create_dir_all(&write_path).unwrap();
        }
        let file_path = self.save_path();
        let mut file = std::fs::File::create(file_path.to_str().unwrap()).unwrap();
        let bytes = serde_json::to_vec_pretty(self.task_db()).unwrap();
        file.write_all(&bytes).unwrap();
    }
    pub fn read_from_file(&mut self) {
        let file_path = self.save_path();
        let file = File::open(file_path.to_str().unwrap());
        if let Ok(file) = file {
            let db = serde_json::from_reader(file);
            if let Ok(db) = db {
                self.task_db = db;
            }
        }
    }

    pub fn save_dir_path(&self) -> PathBuf {
        let home_path = dirs::home_dir().unwrap();
        home_path.join(".venom")
    }
    pub fn save_path(&self) -> PathBuf {
        self.save_dir_path().join("todo.json")
    }

    pub fn toggle_selected_task(&mut self) {
        self.selected_task().borrow_mut().toggle_done();
    }
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    pub fn task_db(&self) -> &TaskDB {
        &self.task_db
    }
    pub fn task_db_mut(&mut self) -> &mut TaskDB {
        &mut self.task_db
    }

    pub fn selected_task_idx(&self) -> usize {
        self.selected_task_idx
    }
    pub fn selected_task(&self) -> Rc<RefCell<Task>> {
        self.task_db.tasks()[self.selected_task_idx].clone()
    }

    pub fn new_example() -> Self {
        let mut result = Self::default();

        let personal_label = Rc::new(RefCell::new(TaskLabel::new(
            "Personal",
            "PSNL",
            RGB::new(0, 0, 200),
        )));
        let research_label = Rc::new(RefCell::new(TaskLabel::new(
            "Research",
            "RSCH",
            RGB::new(200, 30, 200),
        )));
        let class_label = Rc::new(RefCell::new(TaskLabel::new(
            "Class",
            "CLSS",
            RGB::new(200, 120, 0),
        )));

        let date = chrono::NaiveDate::from_ymd_opt(2024, 4, 30).unwrap();
        let time = chrono::NaiveTime::from_hms_opt(15, 30, 0).unwrap();
        let naive_dt = NaiveDateTime::new(date, time);
        let date_2 = naive_dt.and_local_timezone(Local).unwrap();
        let date_1 = Local::now();

        result.task_db.add_label(personal_label);
        result.task_db.add_label(research_label);
        result.task_db.add_label(class_label);

        let task_db = &mut result.task_db;

        let example_task_1 = Task::builder()
            .with_title("Do Homework")
            .with_notes("Problems 1-7 on page 323 of Krane\nProblems 1-2 on page 324")
            .with_priority(Priority::Medium)
            .with_label(task_db.label_by_tag("CLSS"))
            .build();

        let example_task_2 = Task::builder()
            .with_title("Call Doctor")
            .with_notes("(123) 456-7890")
            .with_priority(Priority::Low)
            .with_due_date(Some(date_1))
            .with_label(task_db.label_by_tag("PSNL"))
            .build();

        let example_task_3 = Task::builder()
            .with_title("Write Paper")
            .with_notes("Section 4 requires major revision")
            .with_priority(Priority::High)
            .with_due_date(Some(date_2))
            .with_label(task_db.label_by_tag("RSCH"))
            .build();

        let example_task_4 = Task::builder()
            .with_title("Write Other Paper")
            .with_notes("Oh yeah I have another paper to write")
            .with_due_date(Some(date_2))
            .with_label(task_db.label_by_tag("RSCH"))
            .build();

        let example_task_5 = Task::builder()
            .with_title("Write a todo app in rust")
            .with_notes("Has to be in rust")
            .with_due_date(Some(date_1))
            .with_label(task_db.label_by_tag("PSNL"))
            .with_priority(Priority::High)
            .build();

        task_db.add_task(example_task_1);
        task_db.add_task(example_task_2);
        task_db.add_task(example_task_3);
        task_db.add_task(example_task_4);
        task_db.add_task(example_task_5);

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
        let task = Task::builder().build();
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
