use datetime::LocalDate;
use datetime::LocalTime;

#[derive(Debug, Default)]
pub struct TaskDB {
    tasks: Vec<Task>,
}

impl TaskDB {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn add_default(&mut self) -> &mut Self {
        self.tasks.push(Task::default());
        self
    }

    pub fn task(&self, idx: usize) -> Option<&Task> {
        self.tasks.get(idx)
    }

    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.tasks.push(task);
        self
    }
}

#[derive(Debug, Default)]
pub struct Task {
    title: String,
    priority: Prioity,
    summary: String,
    due_date: Option<LocalDate>,
    due_time: Option<LocalTime>,
}

impl Task {
    pub fn new(title: &str, priority: Prioity) -> Self {
        Self {
            title: title.into(),
            priority,
            summary: "".to_string(),
            due_date: None,
            due_time: None,
        }
    }

    pub fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn priority(&self) -> Prioity {
        self.priority
    }

    pub fn summary(&self) -> &str {
        &self.summary
    }

    pub fn due_date(&self) -> Option<LocalDate> {
        self.due_date
    }
    pub fn due_time(&self) -> Option<LocalTime> {
        self.due_time
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Prioity {
    #[default]
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Default, Clone)]
pub struct TaskBuilder {
    title: String,
    priority: Prioity,
    summary: String,
    due_date: Option<LocalDate>,
    due_time: Option<LocalTime>,
}

impl TaskBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Task {
        Task {
            title: self.title,
            priority: self.priority,
            summary: self.summary,
            due_date: self.due_date,
            due_time: self.due_time,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_summary(mut self, summary: &str) -> Self {
        self.summary = summary.to_string();
        self
    }

    pub fn with_priority(mut self, priority: Prioity) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_due_time(mut self, time: Option<LocalTime>) -> Self {
        self.due_time = time;
        self
    }

    pub fn with_due_date(mut self, date: Option<LocalDate>) -> Self {
        self.due_date = date;
        self
    }
}
