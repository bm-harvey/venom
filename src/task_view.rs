use crate::task::{Task, TaskDB, TaskLabel};
use chrono::Local;
use std::{cell::RefCell, rc::Rc};
//use itertools::Itertools;

#[derive(Default)]
enum CompletedTaskView {
    #[default]
    Seperate,
    Show,
    Hide,
}

#[derive(Default)]
enum SortOption {
    #[default]
    DueDate,
}

#[derive(Default)]
pub struct TaskView {
    completed_task_view: CompletedTaskView,
    sort_option: SortOption,
    current_label: Option<Rc<RefCell<TaskLabel>>>,
    labels: Vec<Rc<RefCell<TaskLabel>>>,
    displayed_tasks: Vec<Rc<RefCell<Task>>>,
}

impl TaskView {
    pub fn toggle_completed_tasks(&mut self) {
        self.completed_task_view = match self.completed_task_view {
            CompletedTaskView::Seperate => CompletedTaskView::Show,
            CompletedTaskView::Show => CompletedTaskView::Hide,
            CompletedTaskView::Hide => CompletedTaskView::Seperate,
        }
    }

    pub fn toggle_selected_label(&mut self) {
        self.current_label = match &self.current_label {
            None => self.labels.first().map(Rc::clone),
            Some(label) => {
                let idx = self
                    .labels
                    .iter()
                    .position(|other| other.borrow().short_name() == label.borrow().short_name());
                match idx {
                    None => None,
                    Some(idx) => {
                        if idx < self.labels.len() - 1 {
                            Some(Rc::clone(&self.labels[idx + 1]))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    pub fn generate_displayed_list(&mut self, db: &TaskDB) {
        self.displayed_tasks = db
            .tasks_iter()
            .filter(|task| match self.completed_task_view {
                CompletedTaskView::Hide => !task.borrow().is_done(),
                _ => true,
            })
            .filter(|task| match &self.current_label {
                None => true,
                Some(label) => {
                    if let Some(task_label) = task.borrow().label() {
                        //Rc::ptr_eq(task_label, label)
                        task_label.borrow().short_name() == label.borrow().short_name()
                    } else {
                        false
                    }
                }
            })
            .cloned()
            .collect();

        self.labels = db.labels().clone();
        self.current_label = match &self.current_label {
            None => None,
            Some(label) => self
                .labels
                .iter()
                .find(|&l| l.borrow().short_name() == label.borrow().short_name())
                .cloned(),
        };

        match self.sort_option {
            SortOption::DueDate => {
                self.displayed_tasks
                    .sort_by_key(|task| task.borrow().due_date().unwrap_or(Local::now()));
            }
        }

        if let CompletedTaskView::Seperate = self.completed_task_view {
            let (v1, v2): (Vec<_>, Vec<_>) = self
                .displayed_tasks
                .iter()
                .cloned()
                .partition(|task| !task.borrow().is_done());
            self.displayed_tasks = v1;
            self.displayed_tasks.extend(v2);
        }
    }

    pub fn num_tasks(&self) -> usize {
        self.displayed_tasks.len()
    }

    pub fn tasks(&self) -> &Vec<Rc<RefCell<Task>>> {
        &self.displayed_tasks
    }

    pub fn labels(&self) -> &Vec<Rc<RefCell<TaskLabel>>> {
        &self.labels
    }

    pub fn has_no_tasks(&self) -> bool {
        return self.tasks().is_empty();
    }
}
