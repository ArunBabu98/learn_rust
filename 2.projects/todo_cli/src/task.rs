use serde::{Deserialize, Serialize};
use std::{fmt, fs, io, path::Path};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum TaskStatus {
    Unfinished,
    Finished,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub list: Vec<Task>,
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { list: Vec::new() }
    }

    pub fn add(&mut self, task: Task) {
        self.list.push(task);
    }

    pub fn remove(&mut self, index: usize) -> Result<(), String> {
        if index < self.list.len() {
            self.list.remove(index);
            Ok(())
        } else {
            Err("Invalid index".into())
        }
    }

    pub fn mark(&mut self, index: usize, status: TaskStatus) -> Result<(), String> {
        if index < self.list.len() {
            self.list[index].status = status;
            Ok(())
        } else {
            Err("Invalid index".into())
        }
    }

    pub fn save_to_json(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_json(path: &str) -> io::Result<Self> {
        if Path::new(path).exists() {
            let data = fs::read_to_string(path)?;
            let list: TaskList = serde_json::from_str(&data)?;
            Ok(list)
        } else {
            Ok(TaskList::new())
        }
    }
}

impl fmt::Display for TaskList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n=== Task List ===")?;
        if self.list.is_empty() {
            writeln!(f, "(No tasks yet)")?;
        }
        for (i, task) in self.list.iter().enumerate() {
            writeln!(
                f,
                "{}. {} [{}]\n   {}",
                i,
                task.name.trim(),
                match task.status {
                    TaskStatus::Finished => "✅ Done",
                    TaskStatus::Unfinished => "❌ Not done",
                },
                task.description.trim()
            )?;
        }
        Ok(())
    }
}
