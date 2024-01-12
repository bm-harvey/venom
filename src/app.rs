use std::rc::Rc;

use crate::task::{self, Task, TaskLabel};
use edtui::{EditorState, EditorView, Lines};
use ratatui::style::Color;
use strum::IntoEnumIterator;
use task::TaskDB;

#[derive(Default)]
pub struct App {
    mode: AppMode,
    should_quit: bool,
    task_db: TaskDB,
    labels: Vec<Rc<TaskLabel>>,
    selected_task_idx: usize,
    edit_task_popup: Option<EditTaskPopup>,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AppMode {
    #[default]
    TaskView,
    EditPopup,
}

#[derive(Default, Clone)]
pub struct EditTaskPopup {
    property: EditableTaskProperty,
    text_editor: EditorState,
    focus: EditTaskFocus,
}

impl EditTaskPopup {
    pub fn property(&self) -> EditableTaskProperty {
        self.property
    }

    pub fn focus(&self) -> EditTaskFocus {
        self.focus
    }
    pub fn set_focus(&mut self, focus: EditTaskFocus) -> &mut Self {
        self.focus = focus;
        self
    }

    pub fn set_property(&mut self, property: EditableTaskProperty) -> &mut Self {
        self.property = property;
        self
    }

    pub fn decrement_property(&mut self) -> &mut Self {
        let properties = EditableTaskProperty::iter().collect::<Vec<_>>();
        let current_idx = properties
            .iter()
            .position(|&p| p == self.property())
            .unwrap();
        let idx = if current_idx == 0 {
            properties.len() - 1
        } else {
            current_idx - 1
        };

        self.property = properties[idx];
        self
    }
    pub fn increment_property(&mut self) -> &mut Self {
        let properties = EditableTaskProperty::iter().collect::<Vec<_>>();
        let current_idx = properties
            .iter()
            .position(|&p| p == self.property())
            .unwrap();
        let idx = if current_idx == properties.len() - 1 {
            0
        } else {
            current_idx + 1
        };
        self.property = properties[idx];
        self
    }

    pub fn load_text(&mut self, text: &str) -> &mut Self {
        self.text_editor = EditorState::new(Lines::from(text));
        self
    }

    pub fn text_editor_widget(&mut self) -> edtui::EditorView {
        EditorView::new(self.text_editor_mut())
    }

    pub fn text_editor_mut(&mut self) -> &mut EditorState {
        &mut self.text_editor
    }
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

#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum EditTaskFocus {
    #[default]
    Fields,
    Edit,
}

use crate::task::Priority;
impl App {
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
    pub fn selected_task(&self) -> Rc<Task> {
        self.task_db.tasks()[self.selected_task_idx].clone()
    }
    pub fn edit_task_popup(&self) -> &Option<EditTaskPopup> {
        &self.edit_task_popup
    }
    pub fn edit_task_popup_mut(&mut self) -> &mut Option<EditTaskPopup> {
        &mut self.edit_task_popup
    }
    pub fn set_edit_task_popup(&mut self, popup: Option<EditTaskPopup>) -> &mut Self {
        self.edit_task_popup = popup;
        self
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

    pub fn mode(&self) -> AppMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: AppMode) {
        self.mode = mode;
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
    }

    pub fn edit_task(&mut self) {
        self.set_edit_task_popup(Some(EditTaskPopup::default()));
        self.mode = AppMode::EditPopup;
    }
}
