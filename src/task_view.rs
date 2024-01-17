use std::{cell::RefCell, rc::Rc};

use chrono::Local;

use crate::task::{Task, TaskDB};
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
    //label: Option<Rc<RefCell<TaskLabel>>>,
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

    pub fn generate_displayed_list(&mut self, db: &TaskDB) {
        self.displayed_tasks = db
            .tasks_iter()
            .filter(|task| match self.completed_task_view {
                CompletedTaskView::Hide => task.borrow().is_done(),
                _ => true,
            })
            .cloned()
            .collect();

        match self.sort_option {
            SortOption::DueDate => {
                self.displayed_tasks
                    .sort_by_key(|task| task.borrow().due_date().unwrap_or(Local::now()));
            }
        }

        if let CompletedTaskView::Seperate = self.completed_task_view {
            let (mut v1, v2): (Vec<_>, Vec<_>) = self
                .displayed_tasks
                .iter()
                .cloned()
                .partition(|task| !task.borrow().is_done());
            v1.extend(v2);
            self.displayed_tasks = v1;
        }
    }

    pub fn tasks(&self) -> &Vec<Rc<RefCell<Task>>> {
        &self.displayed_tasks
    }

    pub fn has_no_tasks(&self) -> bool { 
        return self.tasks().is_empty()
    }
}
