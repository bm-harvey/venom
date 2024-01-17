use crate::edit_labels_popup::EditLabelsPopup;
use crate::edit_task_popup::EditTaskPopup;
use crate::task::{self, Task};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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
    hide_completed: bool,
}

/// The Current Focus of a Venom Application
#[derive(Default, Clone)]
pub enum VenomFocus {
    #[default]
    MainView,
    EditTaskPopup(Rc<RefCell<EditTaskPopup>>),
    EditLabelsPopup(Rc<RefCell<EditLabelsPopup>>),
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
    Label,
    DueDate,
    Notes,
    Priority,
}

impl Venom {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.read_from_file();
        app.task_db_mut().sort_by_date();
        app
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn hide_completed(&self) -> bool {
        self.hide_completed
    }

    pub fn set_hide_completed(&mut self, hide_completed: bool) {
        self.hide_completed = hide_completed;
    }
    pub fn toggle_hide_completed(&mut self) {
        self.hide_completed = !self.hide_completed;
    }

    pub fn remove_selected_task(&mut self) {
        self.task_db.remove_task(self.selected_task_idx());
        self.selected_task_idx = std::cmp::min(self.selected_task_idx, self.task_db.len() - 1);
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

    pub fn add_task_based_on_current(&mut self) {
        let current_task = self.selected_task();
        let current_task_borrow = current_task.borrow();
        let task = Task::builder()
            .with_title(current_task_borrow.title())
            .with_notes(current_task_borrow.notes())
            .with_due_date(current_task_borrow.due_date())
            .with_label(current_task_borrow.label().clone())
            .build();
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
        self.focus = VenomFocus::EditTaskPopup(popup);
    }
    pub fn edit_labels(&mut self) {
        let popup = Rc::new(RefCell::new(EditLabelsPopup::default()));
        popup.borrow_mut().load_labels(self.task_db().labels());
        self.focus = VenomFocus::EditLabelsPopup(popup);
    }
}
