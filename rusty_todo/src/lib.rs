use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};

pub mod state;
pub mod error;

pub use state::AppContext;
pub use error::TodoError;


// --- DATA STRUCTURE ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub description: String,
    pub completed: bool,
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
    Complete(usize),
    Uncomplete(usize),
    Delete(usize),
    Edit(usize, String),
    Version,
}


/// Adds a new task to the list.
pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_task = Task {
        description: description,
        completed: false,
    };
    tasks.push(new_task);
}

pub fn complete_task(tasks: &mut Vec<Task>, id: usize) -> Result<(), TodoError> {
    let index = id - 1;
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

pub fn uncomplete_task(tasks: &mut Vec<Task>, id: usize) -> Result<(), TodoError> {
    let index = id - 1;
    if let Some(task) = tasks.get_mut(index) {
        task.completed = false;
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, id: usize) -> Result<(), TodoError> {
    let index = id - 1;
    if index < tasks.len() {
        tasks.remove(index);
        Ok(())
    } else {
        Err(TodoError::TaskNotFound(id))
    }
}

pub fn edit_task(tasks: &mut Vec<Task>, id: usize, description: String) -> Result<(), TodoError> {
    let index = id - 1;
    if let Some(task) = tasks.get_mut(index) {
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