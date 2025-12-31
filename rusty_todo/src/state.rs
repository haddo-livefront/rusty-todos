use crate::{Task, add_task, complete_task, uncomplete_task, delete_task, edit_task, save_tasks, TodoError, Command, CommandResult};

// --- STATE MACHINE PATTERN ---

/// Trait defining the behavior each state must implement
pub trait AppState: Send {
    /// Handle the current state's logic
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError>;

    /// Return the name of the state for debugging
    fn name(&self) -> &str;
}

// Context holds the application state and delegates behaviour to the current state
pub struct AppContext {
    pub tasks: Vec<Task>,
    pub state: Box<dyn AppState + Send>,
}


impl AppContext {
    /// Create a new context with initial state
    pub fn new(tasks: Vec<Task>) -> Self {
        AppContext {
            tasks,
            state: Box::new(IdleState),
        }
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, state: Box<dyn AppState + Send>) {
        self.state = state
    }

    /// Execute the command by transitioning to the appropriate state and running it
    pub fn execute(&mut self, command: Command) -> Result<CommandResult, TodoError> {
        // Dispatch command to appropriate state
        match command {
            Command::List => self.transition_to(Box::new(ListState)),
            Command::Add(description) => self.transition_to(Box::new(AddState { description })),
            Command::Complete(id) => self.transition_to(Box::new(CompleteState { id })),
            Command::Uncomplete(id) => self.transition_to(Box::new(UncompleteState { id })),
            Command::Delete(id) => self.transition_to(Box::new(DeleteState { id })),
            Command::Edit(id, description) => self.transition_to(Box::new(EditState { id, description })),
            Command::Version => self.transition_to(Box::new(VersionState)),
        }

        // Execute the current state's behavior
        let current_state = std::mem::replace(&mut self.state, Box::new(CompletedState));
        current_state.handle(self)
    }

    /// Get tasks reference
    pub fn tasks_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }
}

/// Initial state - effectively unused now command dispatch is in execute
pub struct IdleState;

impl AppState for IdleState {
    fn handle(&self, _context: &mut AppContext) -> Result<CommandResult, TodoError> {
        Ok(CommandResult::Message(String::new()))
    }

    fn name(&self) -> &str {
        "Idle"
    }
}

/// State for listing all tasks
pub struct ListState;

impl AppState for ListState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        let tasks = context.tasks.clone();
        context.transition_to(Box::new(CompletedState));
        Ok(CommandResult::Tasks(tasks))
    }

    fn name(&self) -> &str {
        "List"
    }
}

/// State for adding a new task
pub struct AddState {
    pub description: String,
}

impl AppState for AddState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        add_task(context.tasks_mut(), self.description.clone());
        context.transition_to(Box::new(SavingState {
            message: "Task added.".to_string(),
        }));
        
        let new_state = std::mem::replace(&mut context.state, Box::new(CompletedState));
        new_state.handle(context)
    }

    fn name(&self) -> &str {
        "Add"
    }
}

/// State for completing a task
pub struct CompleteState {
    pub id: usize,
}

impl AppState for CompleteState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        complete_task(context.tasks_mut(), self.id)?;
        context.transition_to(Box::new(SavingState {
            message: "Task marked as complete.".to_string(),
        }));
        
        let new_state = std::mem::replace(&mut context.state, Box::new(CompletedState));
        new_state.handle(context)
    }

    fn name(&self) -> &str {
        "Complete"
    }
}

/// State for marking a task as incomplete
pub struct UncompleteState {
    pub id: usize,
}

impl AppState for UncompleteState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        uncomplete_task(context.tasks_mut(), self.id)?;
        context.transition_to(Box::new(SavingState {
            message: "Task marked as incomplete.".to_string(),
        }));
        
        let new_state = std::mem::replace(&mut context.state, Box::new(CompletedState));
        new_state.handle(context)
    }

    fn name(&self) -> &str {
        "Uncomplete"
    }
}

/// State for deleting a task
pub struct DeleteState {
    pub id: usize,
}

impl AppState for DeleteState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        delete_task(context.tasks_mut(), self.id)?;
        context.transition_to(Box::new(SavingState {
            message: "Task deleted.".to_string(),
        }));
        
        let new_state = std::mem::replace(&mut context.state, Box::new(CompletedState));
        new_state.handle(context)
    }

    fn name(&self) -> &str {
        "Delete"
    }
}

/// State for editing a task
pub struct EditState {
    pub id: usize,
    pub description: String,
}

impl AppState for EditState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        edit_task(context.tasks_mut(), self.id, self.description.clone())?;
        context.transition_to(Box::new(SavingState {
            message: "Task updated.".to_string(),
        }));
        
        let new_state = std::mem::replace(&mut context.state, Box::new(CompletedState));
        new_state.handle(context)
    }

    fn name(&self) -> &str {
        "Edit"
    }
}

/// State for saving tasks to disk
pub struct SavingState {
    pub message: String,
}

impl AppState for SavingState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        save_tasks(context.tasks_mut()).map_err(TodoError::from)?;
        context.transition_to(Box::new(CompletedState));
        Ok(CommandResult::Message(self.message.clone()))
    }

    fn name(&self) -> &str {
        "Saving"
    }
}

/// Completed tasks state
pub struct CompletedState;

impl AppState for CompletedState {
    fn handle(&self, _context: &mut AppContext) -> Result<CommandResult, TodoError> {
        Ok(CommandResult::Message(String::new()))
    }

    fn name(&self) -> &str {
        "Completed"
    }
}

/// State for reporting the version
pub struct VersionState;

impl AppState for VersionState {
    fn handle(&self, context: &mut AppContext) -> Result<CommandResult, TodoError> {
        let version = env!("CARGO_PKG_VERSION").to_string();
        context.transition_to(Box::new(CompletedState));
        Ok(CommandResult::Message(version))
    }

    fn name(&self) -> &str {
        "Version"
    }
}

