use crate::edit_labels_popup::EditLabelsPopup;
use crate::edit_task_popup::EditTaskPopup;
use crate::task::{self, Task};
use crate::task_view::TaskView;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use strum;
use task::TaskDB;

/// The global app state
#[derive(Default)]
pub struct Venom {
    /// Checked by the event loop
    should_quit: bool,
    /// The task and labels database
    task_db: TaskDB,
    /// The current index currently hilighted
    selected_task_idx: usize,
    /// What is currently focused by the app
    focus: VenomFocus,
    /// This is to be removed in favor of task views
    hide_completed: bool,
    task_view: TaskView,
}

/// The Current Focus of a Venom Application
#[derive(Default, Clone)]
pub enum VenomFocus {
    /// The MainView is the primary way to be looking at the tasks unless you are editing
    #[default]
    MainView,
    /// A popup to edit a task
    EditTaskPopup(Rc<RefCell<EditTaskPopup>>),
    /// A popup to edit the available labels
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
    /// Use this as the save path (from the home directory) for save and config files
    const SAVE_DIR_STR: &'static str = ".venom";
    /// Use this as the default save file name
    const DEFAULT_SAVE_FILE_STR: &'static str = "todo.json";

    pub fn new() -> Self {
        let mut app = Self::default();
        app.read_from_file();
        app.update_view();
        app
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

    pub fn save_path(&self) -> PathBuf {
        self.save_dir_path().join(Self::DEFAULT_SAVE_FILE_STR)
    }

    pub fn save_dir_path(&self) -> PathBuf {
        let home_path = dirs::home_dir().unwrap();
        home_path.join(Self::SAVE_DIR_STR)
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

    /// To do every tick
    pub fn tick(&self) {}

    /// Flag that the aplication should quit
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    /// Check if the aplication should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Remove the selected task in the current view from the database
    pub fn remove_selected_task(&mut self) {
        let task = self.selected_task();
        self.task_db.remove_task(&task);
        self.selected_task_idx =
            std::cmp::min(self.selected_task_idx, self.task_db.num_tasks() - 1);
    }

    /// task view
    pub fn task_view(&self) -> &TaskView {
        &self.task_view
    }

    /// task view
    pub fn task_view_mut(&mut self) -> &mut TaskView {
        &mut self.task_view
    }

    /// return a shared reference of the task database
    pub fn task_db(&self) -> &TaskDB {
        &self.task_db
    }

    /// return a mutable reference to the task database
    pub fn task_db_mut(&mut self) -> &mut TaskDB {
        &mut self.task_db
    }

    /// idx of the selected in the current view
    pub fn selected_task_idx(&self) -> usize {
        self.selected_task_idx
    }

    /// The task in the current view which is indexed
    pub fn selected_task(&self) -> Rc<RefCell<Task>> {
        self.task_view.tasks()[self.selected_task_idx].clone()
    }

    /// mark the current highlighted task as done / not done.
    pub fn toggle_selected_task(&mut self) {
        self.selected_task().borrow_mut().toggle_done();
    }

    /// The current focus of the main application
    pub fn focus(&self) -> &VenomFocus {
        &self.focus
    }

    /// Set the focus of the application. Note that if the old focus stored a popup, that popup
    /// will get lost and replaced by the new focus. This is currenlty expected to remain the mode
    /// of operations, but if this were a GUI then that might not be the best way to run.
    pub fn set_focus(&mut self, mode: VenomFocus) {
        self.focus = mode;
    }

    /// Effectively move down the list, looping back at the top if nesacary
    pub fn increment_task_idx(&mut self) {
        if self.task_db.has_no_tasks() {
            self.selected_task_idx = 0;
            return;
        }

        let overflow = self.selected_task_idx + 1 == self.task_view.num_tasks();
        self.selected_task_idx = if overflow {
            0
        } else {
            self.selected_task_idx + 1
        }
    }

    /// Effectively move up the list, looping back at the bottom if nesacary
    pub fn decrement_task_idx(&mut self) {
        if self.task_db.has_no_tasks() {
            self.selected_task_idx = 0;
            return;
        }

        let underflow = self.selected_task_idx == 0;
        self.selected_task_idx = if underflow {
            self.task_view.num_tasks() - 1
        } else {
            self.selected_task_idx - 1
        }
    }
    pub fn update_view(&mut self) {
        self.task_view.generate_displayed_list(&self.task_db);

        self.selected_task_idx = {
            let num_in_view = self.task_view().tasks().len();
            if num_in_view == 0 {
                0
            } else {
                std::cmp::min(self.selected_task_idx(), self.task_view().tasks().len() - 1)
            }
        }
    }

    pub fn toggle_completed_task_view(&mut self) {
        self.task_view_mut().toggle_completed_tasks();
        self.update_view();
    }

    pub fn toggle_selected_label(&mut self) {
        self.task_view_mut().toggle_selected_label();
        self.update_view();
    }

    /// Add a blank task and then open up the editing popup for it
    pub fn add_task(&mut self) {
        let task = Rc::new(RefCell::new(Task::default()));
        self.task_db.add_task(Rc::clone(&task));
        self.selected_task_idx = 0;
        self.update_view();
        self.edit_task(task);
    }

    /// Copy a task and then edit the new one
    pub fn add_task_based_on_current(&mut self) {
        let current_task = self.selected_task();
        let current_task_borrow = current_task.borrow();
        let task = Task::builder()
            .with_title(current_task_borrow.title())
            .with_notes(current_task_borrow.notes())
            .with_due_date(current_task_borrow.due_date())
            .with_label(current_task_borrow.label().clone())
            .build_rcc();
        self.task_db.add_task(Rc::clone(&task));
        self.selected_task_idx = 0;
        self.edit_task(task);
        self.update_view();
    }

    /// Open a task edit popup
    pub fn edit_current_task(&mut self) {
        let task = self.selected_task();
        let popup = Rc::new(RefCell::new(EditTaskPopup::new(&task)));
        let property = popup.borrow().property();
        popup
            .borrow_mut()
            .load_text(&self.selected_task().borrow().text_to_edit(property));
        self.focus = VenomFocus::EditTaskPopup(popup);
    }
    /// Open a task edit popup
    pub fn edit_task(&mut self, task: Rc<RefCell<Task>>) {
        let popup = Rc::new(RefCell::new(EditTaskPopup::new(&task)));
        let property = popup.borrow().property();
        popup
            .borrow_mut()
            .load_text(&task.borrow().text_to_edit(property));
        self.focus = VenomFocus::EditTaskPopup(popup);
    }

    /// Open a label edit popup
    pub fn edit_labels(&mut self) {
        let popup = Rc::new(RefCell::new(EditLabelsPopup::default()));
        popup.borrow_mut().load_labels(self.task_db().labels());
        self.focus = VenomFocus::EditLabelsPopup(popup);
    }

    // todo: remove in favor of views
    pub fn hide_completed(&self) -> bool {
        self.hide_completed
    }
    // todo: remove in favor of views
    pub fn set_hide_completed(&mut self, hide_completed: bool) {
        self.hide_completed = hide_completed;
    }
}
