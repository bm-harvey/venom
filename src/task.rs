use crate::venom::EditableTaskProperty;
use chrono::{DateTime, Local};
use chrono::{Datelike, Timelike};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::ptr::eq;
use std::rc::Rc;
use std::str::FromStr;

/// Data structure to keep track of the tasks and the labels attatched to them.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskDB {
    /// list of all tasks. Tasks are stored as [`Rc<RefCell<Task>>`] for shared mutability - this
    /// might change in the future.
    tasks: Vec<Rc<RefCell<Task>>>,
    /// list of all labels. Labels are stored as [`Rc<RefCell<Task>>`] for shared mutability - this
    /// is unlikely to change in the future. Lablels need to be editable from a task that holds
    /// them, and the app itself... i think.
    labels: Vec<Rc<RefCell<TaskLabel>>>,
}

impl TaskDB {
    pub fn new() -> Self {
        Self::default()
    }

    /// Remove a task by idx
    pub fn remove_task(&mut self, task: &Rc<RefCell<Task>>) {
        let rm_idx = self.tasks.iter().position(|t| Rc::ptr_eq(t, task));

        if let Some(rm_idx) = rm_idx {
            self.tasks.remove(rm_idx);
        }
    }

    /// Remove a label. To fully remove a label, one needs to remove it from all of the tasks as
    /// well.
    pub fn remove_label(&mut self, tag: &str) {
        let tag = TaskLabel::generate_short_name(tag);

        self.labels.retain(|l| l.borrow().short_name() != tag);

        self.tasks
            .iter()
            .filter(|task| {
                let binding = task.borrow();
                let label = binding.label();
                match label {
                    Some(label) => label.borrow().short_name() == tag,
                    None => false,
                }
            })
            .for_each(|task| task.borrow_mut().remove_label());
    }

    /// Labels list
    pub fn labels(&self) -> &Vec<Rc<RefCell<TaskLabel>>> {
        &self.labels
    }

    /// Mutable labels list
    pub fn labels_mut(&mut self) -> &mut Vec<Rc<RefCell<TaskLabel>>> {
        &mut self.labels
    }

    /// Find the first label by its tag
    pub fn label_by_tag(&self, tag: &str) -> Option<Rc<RefCell<TaskLabel>>> {
        let short_name = {
            let mut result = [' '; TaskLabel::LABEL_LEN];
            for (idx, val) in tag.chars().enumerate() {
                if idx >= TaskLabel::LABEL_LEN {
                    break;
                }
                result[idx] = val;
            }
            result
        };
        self.labels
            .iter()
            .find(|&l| l.borrow().short_name() == short_name)
            .cloned()
    }

    pub fn tasks(&self) -> &Vec<Rc<RefCell<Task>>> {
        &self.tasks
    }

    pub fn tasks_iter(&self) -> impl Iterator<Item = &Rc<RefCell<Task>>> {
        self.tasks.iter()
    }
    pub fn tasks_mut(&mut self) -> &mut Vec<Rc<RefCell<Task>>> {
        &mut self.tasks
    }

    pub fn num_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn has_no_tasks(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn has_tasks(&self) -> bool {
        !self.has_no_tasks()
    }

    pub fn add_default(&mut self) -> &mut Self {
        self.tasks.push(Task::default_rcc());
        self
    }

    /// pull a task by cloning the [`Rc`] by index.
    pub fn task(&self, idx: usize) -> Option<Rc<RefCell<Task>>> {
        match idx {
            idx if idx < self.num_tasks() => Some(Rc::clone(&self.tasks[idx])),
            _ => None,
        }
    }

    /// insert a task
    pub fn add_raw_task(&mut self, task: Task) -> &mut Self {
        self.tasks.push(Rc::new(RefCell::new(task)));
        self
    }

    /// insert a task
    pub fn add_task(&mut self, task: Rc<RefCell<Task>>) -> &mut Self {
        self.tasks.push(task);
        self
    }

    /// insert a label
    pub fn add_raw_label(&mut self, label: TaskLabel) -> &mut Self {
        self.labels.push(Rc::new(RefCell::new(label)));
        self
    }

    /// add a label from a constructed [`Rc<RefCell<_>>`]
    pub fn add_label(&mut self, label: Rc<RefCell<TaskLabel>>) -> &mut Self {
        self.labels.push(label);
        self
    }

    /// Sort the tasks by doneness and date. This is to be removed in favor of views
    /// todo
    pub fn sort_by_date(&mut self) {
        self.tasks.sort_by(|t1, t2| {
            let (t1, t2) = (t1.borrow(), t2.borrow());
            match (t1.is_done(), t2.is_done()) {
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (_, _) => {
                    let dt1 = t1.due_date().unwrap_or(Local::now());
                    let dt2 = t2.due_date().unwrap_or(Local::now());
                    dt1.cmp(&dt2)
                }
            }
        })
    }
}

/// Task that can be marked done or not
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    title: String,
    priority: Priority,
    notes: String,
    due_date: Option<DateTime<chrono::Local>>,
    label: Option<Rc<RefCell<TaskLabel>>>,
    done: bool,
}

impl Task {
    pub fn new(title: &str, priority: Priority) -> Self {
        Self {
            title: title.into(),
            priority,
            notes: "".to_string(),
            due_date: None,
            label: None,
            done: false,
        }
    }

    pub fn default_rcc() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn remove_label(&mut self) {
        self.label = None;
    }

    pub fn text_to_edit(&self, property: EditableTaskProperty) -> String {
        match property {
            EditableTaskProperty::Title => self.title().to_string(),
            EditableTaskProperty::Notes => self.notes().to_string(),
            EditableTaskProperty::Priority => self.priority().to_string(),
            EditableTaskProperty::DueDate => {
                format!("{} {}", self.date_string(), self.time_string())
            }
            EditableTaskProperty::Label => match self.label() {
                Some(label) => label.borrow().short_name().iter().collect(),
                None => "".to_string(),
            },
        }
    }
    pub fn set_property_from_str(&mut self, property: EditableTaskProperty, value: &str) {
        match property {
            EditableTaskProperty::Title => {
                self.set_title(value);
            }
            EditableTaskProperty::Notes => {
                self.set_notes(value);
            }
            EditableTaskProperty::DueDate => {
                self.set_date_str(value);
            }
            EditableTaskProperty::Priority => {
                match value {
                    "None" => {
                        self.set_priority(Priority::None);
                    }
                    "Low" => {
                        self.set_priority(Priority::Low);
                    }
                    "Medium" => {
                        self.set_priority(Priority::Medium);
                    }
                    "High" => {
                        self.set_priority(Priority::High);
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }

    pub fn set_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;
        self
    }

    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn label(&self) -> &Option<Rc<RefCell<TaskLabel>>> {
        &self.label
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn toggle_done(&mut self) -> &mut Self {
        self.done = !self.done;
        self
    }
    pub fn priority(&self) -> Priority {
        self.priority
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn due_date(&self) -> Option<DateTime<Local>> {
        self.due_date
    }

    pub fn date_string(&self) -> String {
        match self.due_date {
            None => "".to_string(),
            Some(date) => {
                let year_string = date.date_naive().year().to_string();
                let month_string = match date.date_naive().month() {
                    1 => "Jan",
                    2 => "Feb",
                    3 => "Mar",
                    4 => "Apr",
                    5 => "May",
                    6 => "Jun",
                    7 => "Jul",
                    8 => "Aug",
                    9 => "Sep",
                    10 => "Oct",
                    11 => "Nov",
                    12 => "Dec",
                    _ => unreachable!(),
                }
                .to_string();

                let day_string = date.day().to_string();

                format!("{:>2} {} {}", day_string, month_string, year_string)
            }
        }
    }
    pub fn time_string(&self) -> String {
        if let Some(date) = self.due_date() {
            let time = date.time();
            let hour_string = time.hour().to_string();
            let min_string = time.minute().to_string();
            format!("{:0>2}:{:0>2}", hour_string, min_string)
        } else {
            String::new()
        }
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn set_label(&mut self, label: Option<Rc<RefCell<TaskLabel>>>) -> &mut Self {
        self.label = label;
        self
    }

    pub fn set_date_str(&mut self, date: &str) -> &mut Self {
        let words = date.split_whitespace().collect::<Vec<_>>();

        let num_words = words.len();

        match num_words {
            0 => self.set_no_date(),
            4 => {
                let day = words[0].parse::<u32>();
                let month = match words[1] {
                    "Jan" => Some(1),
                    "Feb" => Some(2),
                    "Mar" => Some(3),
                    "Apr" => Some(4),
                    "May" => Some(5),
                    "Jun" => Some(6),
                    "Jul" => Some(7),
                    "Aug" => Some(8),
                    "Sep" => Some(9),
                    "Oct" => Some(10),
                    "Nov" => Some(11),
                    "Dec" => Some(12),
                    _ => None,
                };
                let year = words[2].parse::<i32>();

                let new_date = if day.is_err() || month.is_none() || year.is_err() {
                    self.due_date().map(|date| date.date_naive())
                } else {
                    let day = day.unwrap();
                    let month = month.unwrap();
                    let year = year.unwrap();
                    chrono::NaiveDate::from_ymd_opt(year, month, day)
                };

                let time = words[3].split(':').collect::<Vec<_>>();
                let new_time = if time.len() != 2 {
                    self.due_date().map(|date| date.time())
                } else {
                    let hour = time[0].parse::<u32>();
                    let minute = time[1].parse::<u32>();

                    if hour.is_err() || minute.is_err() {
                        self.due_date().map(|date| date.time())
                    } else {
                        let hour = hour.unwrap();
                        let minute = minute.unwrap();
                        chrono::NaiveTime::from_hms_opt(hour, minute, 0)
                    }
                };

                if new_date.is_some() && new_time.is_some() {
                    let naive_dt = chrono::NaiveDateTime::new(new_date.unwrap(), new_time.unwrap());
                    let date_time = naive_dt.and_local_timezone(Local).unwrap();
                    self.set_date(&date_time)
                } else {
                    self
                }
            }
            _ => self,
        }
    }
    pub fn set_date(&mut self, date: &DateTime<Local>) -> &mut Self {
        self.due_date = Some(*date);
        self
    }
    pub fn set_no_date(&mut self) -> &mut Self {
        self.due_date = None;
        self
    }
    pub fn set_notes(&mut self, notes: &str) -> &mut Self {
        self.notes = notes.to_string();
        self
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, strum::Display)]
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

    pub fn formatting(&self) -> (Color, String) {
        match self {
            Self::None => (Color::default(), self.to_string()),
            Self::Low => (Color::Green, self.to_string()),
            Self::Medium => (Color::Yellow, self.to_string()),
            Self::High => (Color::Red, self.to_string()),
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

    pub fn build_rcc(self) -> Rc<RefCell<Task>> {
        Rc::new(RefCell::new(self.task))
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

    pub fn with_due_date(mut self, date: Option<DateTime<Local>>) -> Self {
        self.task.due_date = date;
        self
    }

    pub fn with_label(mut self, label: Option<Rc<RefCell<TaskLabel>>>) -> Self {
        self.task.label = label;
        self
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn rat_color(&self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }
}
impl Default for RGB {
    fn default() -> Self {
        RGB::new(200, 200, 200)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TaskLabel {
    short_name: [char; Self::LABEL_LEN],
    long_name: String,
    color_str: String,
}

impl TaskLabel {
    pub const LABEL_LEN: usize = 4;

    pub fn new(long_name: &str, short_name: &str, color_str: &str) -> Self {
        let long_name = long_name.to_string();

        let short_name = Self::generate_short_name(short_name);

        Self {
            short_name,
            long_name,
            color_str: color_str.to_string(),
        }
    }

    pub fn generate_short_name(text: &str) -> [char; Self::LABEL_LEN] {
        let mut result = [' '; Self::LABEL_LEN];
        for (idx, val) in text.chars().enumerate() {
            if idx >= Self::LABEL_LEN {
                break;
            }
            result[idx] = val;
        }
        result
    }
    pub fn set_color(&mut self, color: &str) -> &mut Self {
        self.color_str = color.to_string();
        self
    }

    pub fn set_tag(&mut self, tag: &str) -> &mut Self {
        self.short_name = Self::generate_short_name(tag);
        self
    }
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.long_name = name.to_string();
        self
    }

    pub fn color_string(&self) -> &str {
        &self.color_str
    }
    pub fn color(&self) -> Color {
        Color::from_str(&self.color_str).unwrap_or(Color::default())
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
            Style::default().fg(self.color()),
        )
    }
}
