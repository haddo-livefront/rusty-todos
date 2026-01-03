use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use uuid::Uuid;

pub mod state;
pub mod error;

pub use state::AppContext;
pub use error::TodoError;

// --- DATA STRUCTURE ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    #[serde(default = "generate_id")]
    pub id: String,
    pub description: String,
    pub completed: bool,
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Result of execution a command
pub enum CommandResult {
    Message(String),
    Tasks(Vec<Task>),
}

/// Commands that the application can execute
pub enum Command {
    List,
    Add(String),
    Complete(String),
    Uncomplete(String),
    Delete(String),
    Edit(String, String),
    Version,
}

/// Adds a new task to the list.
pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_task = Task {
        id: generate_id(),
        description,
        completed: false,
    };
    tasks.push(new_task);
}

pub fn complete_task(tasks: &mut Vec<Task>, id: String) -> Result<(), TodoError> {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

pub fn uncomplete_task(tasks: &mut Vec<Task>, id: String) -> Result<(), TodoError> {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = false;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: String) -> Result<(), TodoError> {
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.swap_remove(index);
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

pub fn edit_task(tasks: &mut Vec<Task>, id: String, description: String) -> Result<(), TodoError> {
    if let Some(task) = tasks.iter_mut().find(|t|t.id == id) {
        task.description = description;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

/// Loads the list of tasks from "tasks.json".
pub fn load_tasks() -> Result<Vec<Task>, std::io::Error> {
    let file = File::open("tasks.json")?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

/// Saves the list of tasks to "tasks.json".
pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    let mut file = File::create("tasks.json")?;
    let json_data = serde_json::to_string_pretty(tasks)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}