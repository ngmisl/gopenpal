use crate::error::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub priority: String,
    pub due_date: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

pub struct TaskManager {
    file_path: PathBuf,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            file_path: PathBuf::from("world/tasks.md"),
        }
    }

    fn ensure_file_exists(&self) -> Result<()> {
        if !self.file_path.exists() {
            if let Some(parent) = self.file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::File::create(&self.file_path)?;
            writeln!(file, "# Tasks\n")?;
        }
        Ok(())
    }

    fn parse_tasks(&self) -> Result<Vec<Task>> {
        self.ensure_file_exists()?;
        let content = fs::read_to_string(&self.file_path)?;
        let mut tasks = Vec::new();
        let mut current_id = 1;

        for line in content.lines() {
            if line.trim().starts_with("- [") {
                let completed = line.contains("- [x]");

                // Extract content after the checkbox
                let content = line
                    .trim_start_matches("- [ ] ")
                    .trim_start_matches("- [x] ")
                    .trim();

                let mut title = content.to_string();
                let mut priority = "medium".to_string();
                let mut due_date = None;
                let mut created_at = Local::now().to_rfc3339();

                // Parse tags from the end of the string backwards
                // Supported tags: [priority: low|medium|high], [due: YYYY-MM-DD], [cre: date]
                // We keep stripping matching tags until we hit the title

                let mut changed = true;
                while changed {
                    changed = false;

                    if let Some(start_idx) = title.rfind('[') {
                        if let Some(end_idx) = title.rfind(']') {
                            if end_idx > start_idx && end_idx == title.len() - 1 {
                                let tag_content = &title[start_idx + 1..end_idx];
                                let mut handled = false;

                                if let Some(p) = tag_content.strip_prefix("priority: ") {
                                    priority = p.trim().to_string();
                                    handled = true;
                                } else if let Some(d) = tag_content.strip_prefix("due: ") {
                                    due_date = Some(d.trim().to_string());
                                    handled = true;
                                } else if let Some(c) = tag_content.strip_prefix("cre: ") {
                                    created_at = c.trim().to_string();
                                    handled = true;
                                }

                                if handled {
                                    title = title[..start_idx].trim_end().to_string();
                                    changed = true;
                                }
                            }
                        }
                    }
                }

                tasks.push(Task {
                    id: current_id,
                    title,
                    priority,
                    due_date,
                    completed,
                    created_at,
                });
                current_id += 1;
            }
        }
        Ok(tasks)
    }

    fn write_tasks(&self, tasks: &[Task]) -> Result<()> {
        let mut content = String::from("# Tasks\n\n");
        for task in tasks {
            let check = if task.completed { "x" } else { " " };
            let mut line = format!("- [{}] {}", check, task.title);

            if task.priority != "medium" {
                line.push_str(&format!(" [priority: {}]", task.priority));
            }
            if let Some(due) = &task.due_date {
                line.push_str(&format!(" [due: {}]", due));
            }
            // We generally don't show creation date in the markdown to keep it clean,
            // but we could persist it if needed. For now, let's keep it simple for the user.

            content.push_str(&line);
            content.push('\n');
        }
        fs::write(&self.file_path, content)?;
        Ok(())
    }

    pub fn create_task(&self, title: &str, priority: &str, due_date: Option<&str>) -> Result<Task> {
        let mut tasks = self.parse_tasks()?;
        let id = tasks.len() + 1;

        let new_task = Task {
            id,
            title: title.to_string(),
            priority: priority.to_string(),
            due_date: due_date.map(String::from),
            completed: false,
            created_at: Local::now().to_rfc3339(),
        };

        tasks.push(new_task.clone());
        self.write_tasks(&tasks)?;
        Ok(new_task)
    }

    pub fn list_tasks(&self, filter: &str) -> Result<Vec<Task>> {
        let tasks = self.parse_tasks()?;

        match filter {
            "all" => Ok(tasks),
            "pending" => Ok(tasks.into_iter().filter(|t| !t.completed).collect()),
            "completed" => Ok(tasks.into_iter().filter(|t| t.completed).collect()),
            "high-priority" => Ok(tasks
                .into_iter()
                .filter(|t| !t.completed && (t.priority == "high" || t.priority == "urgent"))
                .collect()),
            "today" => {
                let today = Local::now().format("%Y-%m-%d").to_string();
                Ok(tasks
                    .into_iter()
                    .filter(|t| !t.completed && t.due_date.as_deref() == Some(&today))
                    .collect())
            }
            _ => Ok(tasks.into_iter().filter(|t| !t.completed).collect()), // Default to pending
        }
    }

    pub fn complete_task(&self, id: usize) -> Result<Option<Task>> {
        let mut tasks = self.parse_tasks()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            let completed_task = task.clone();
            self.write_tasks(&tasks)?;
            Ok(Some(completed_task))
        } else {
            Ok(None)
        }
    }

    pub fn update_task(&self, id: usize, field: &str, value: &str) -> Result<Option<Task>> {
        let mut tasks = self.parse_tasks()?;
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            match field {
                "title" => task.title = value.to_string(),
                "priority" => task.priority = value.to_string(),
                "due_date" => task.due_date = Some(value.to_string()),
                _ => return Ok(None),
            }
            let updated_task = task.clone();
            self.write_tasks(&tasks)?;
            Ok(Some(updated_task))
        } else {
            Ok(None)
        }
    }

    pub fn delete_task(&self, id: usize) -> Result<bool> {
        let mut tasks = self.parse_tasks()?;
        let len_before = tasks.len();
        tasks.retain(|t| t.id != id);

        if tasks.len() < len_before {
            self.write_tasks(&tasks)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
