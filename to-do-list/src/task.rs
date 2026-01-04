struct Task {
    id: usize,
    description: String,
    done: bool,
}

pub struct TaskManager {
    next_id: usize,
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task {
            id: self.next_id,
            description,
            done: false,
        };
        self.tasks.push(task);
        println!("Task added successfully. ID : {}", self.next_id);
        self.next_id += 1;
    }

    pub fn show_list(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for task in &self.tasks {
                println!("[{}] {}       {}", task.id, task.description, if task.done {"Done"} else {"Pending"});
            }
        }
    }

    pub fn done_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.done = true;
            println!("Task {} was done.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    pub fn search_task(&self, id: usize) {
        if let Some(task) = self.tasks.iter().find(|t| t.id == id) {
            println!("[{}] {}       {}", task.id, task.description, if task.done {"Done"} else {"Pending"});
        } else {
            println!("Task with ID {} not found.", id);
        }
    }


    pub fn delete_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter_mut().position(|t| t.id == id) {
            self.tasks.remove(pos);
            println!("Task {} was removed.", id);
        } else {
            println!("Task with ID {} not found.", id);
        }
    }

    pub fn clear_all(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
        } else { 
            self.tasks.clear();
            self.next_id = 1;
            println!("Deleted all tasks.");
        }
    }
}