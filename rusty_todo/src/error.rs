use std::fmt;

#[derive(Debug)]
pub enum TodoError {
    IoError(std::io::Error),
    ParseError(serde_json::Error),
    InvalidCommand(String),
    MissingArgument(String),
    InvalidId(String),
    TaskNotFound(usize),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::IoError(e) => write!(f, "IO Error: {}", e),
            TodoError::ParseError(e) => write!(f, "Parse Error: {}", e),
            TodoError::InvalidCommand(cmd) => write!(f, "Unknown command '{}'. Run with no arguments for usage.", cmd),
            TodoError::MissingArgument(arg) => write!(f, "Missing argument: {}. Run with no arguments for usage.", arg),
            TodoError::InvalidId(id) => write!(f, "Invalid task ID '{}'. Please provide a number.", id),
            TodoError::TaskNotFound(id) => write!(f, "No task found with ID {}.", id),
        }
    }
}

impl std::error::Error for TodoError {}

impl From<std::io::Error> for TodoError {
    fn from(err: std::io::Error) -> TodoError {
        TodoError::IoError(err)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> TodoError {
        TodoError::ParseError(err)
    }
}
