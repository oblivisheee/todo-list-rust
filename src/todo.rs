pub struct TodoList {
    pub tasks: Vec<String>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    pub fn add(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn complete(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    pub fn print(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }
}
