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

    pub fn add_task(&mut self, title: &str, priority: Prioity) -> &mut Self {
        self.tasks.push(Task::new(title, priority));
        self
    }
}

#[derive(Debug, Default)]
pub struct Task {
    title: String,
    priority: Prioity,
}

impl Task {
    pub fn new(title: &str, priority: Prioity) -> Self {
        Self {
            title: title.into(),
            priority 
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn priority(&self) -> Prioity {
        self.priority
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Prioity {
    #[default]
    Low,
    Medium,
    High,
}
