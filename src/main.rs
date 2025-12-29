// These `use` statements bring in necessary components from Rust's standard library
// and the external crates that have been added to Cargo.toml.

// `serde` traits for serialization (writing data to a format like JSON) and
// deserialization (reading data from a format like JSON).
use serde::{Deserialize, Serialize};
// `std::env` is used to access command-line arguments.
use std::env;
// `std::fs` provides functions for file system operations, like reading and writing files.
use std::fs::{File};
// `std::io` contains Rust's I/O functionality. Need `Write` for saving files
// and `BufReader` for reading.
use std::io::{BufReader, Write};
// `std::process` is used to exit the program, for example, when invalid input is given.
use std::process;

// --- DATA STRUCTURE ---
// This is the blueprint for a single to do item.
// The `#[derive(...)]` is an attribute that tells the Rust compiler to automatically
// generate code for certain traits.
#[derive(Serialize, Deserialize, Debug)] // Debug allows us to print the struct nicely using `{:?}`
struct Task {
    description: String,
    completed: bool,
}

// --- STATE MACHINE PATTERN ---

/// Trait defining the behavior each state must implement
trait AppState {

    /// Handle the current state's logic
    fn handle(&self, context: &mut AppContext, args: &[String]);

    /// Return the name of the state for debugging
    fn name(&self) -> &str;
}

// Context holds the application state and delegates behaviour to the current state
struct AppContext {
    tasks: Vec<Task>,
    state: Box<dyn AppState>,
}

impl AppContext {
    /// Create a new context with initial state
    fn new(tasks: Vec<Task>) -> Self {
        AppContext {
            tasks,
            state: Box::new(IdleState),
        }
    }

    /// Transition to a new state
    fn transition_to(&mut self, state: Box<dyn AppState>) {
        // Put a guard to make sure you can go from the order of the app states: idle -> list/add/complete
        println!("[State Transition] -> {}", state.name());
        self.state = state
    }

    /// Execute the current state's behavior
    fn execute(&mut self, args: &[String]) {
        // Take ownership of the current state, replacing it with a placeholder
        // This allows us to call handle with a mutable reference to self
        let current_state = std::mem::replace(&mut self.state, Box::new(CompletedState));
        current_state.handle(self, args);
    }

    /// Get tasks reference
    fn tasks_mut(&mut self) -> &mut Vec<Task> {
        &mut self.tasks
    }
}

/// Initial state - determines which operation to perform
struct IdleState;

impl AppState for IdleState {
    fn handle(&self, context: &mut AppContext, args: &[String]) {
        if args.len() < 2 {
            print_usage();
            process::exit(1);
        }

        let command = &args[1];
        
        // Transition to appropriate state based on command
        match command.as_str() {
            "list" => context.transition_to(Box::new(ListState)),
            "add" => context.transition_to(Box::new(AddState)),
            "done" => context.transition_to(Box::new(CompleteState)),
            _ => {
                println!("Error: Unknown command '{}'", command);
                print_usage();
                process::exit(1);
            }
        }
        
        // Execute the new state
        context.execute(args);
    }

    fn name(&self) -> &str {
        "Idle"
    }
}

/// State for listing all tasks
struct ListState;

impl AppState for ListState {
    fn handle(&self, context: &mut AppContext, _args: &[String]) {

        list_tasks(&context.tasks);
        context.transition_to(Box::new(CompletedState));
    }

    fn name(&self) -> &str {
        "List"
    }
}

/// State for adding a new task
struct AddState;

impl AppState for AddState {
    fn handle(&self, context: &mut AppContext, args: &[String]) {
        let description = args.get(2).cloned().unwrap_or_else(|| {
            println!("Error: 'add' command requires a description.");
            print_usage();
            process::exit(1);
        });

        add_task(context.tasks_mut(), description);
        context.transition_to(Box::new(SavingState {
            message: "Task added.".to_string(),
        }));
        context.execute(args);
    }

    fn name(&self) -> &str {
        "Add"
    }
}

/// State for completing a task
struct CompleteState;

impl AppState for CompleteState {
    fn handle(&self, context: &mut AppContext, args: &[String]) {
        let id_str = args.get(2).unwrap_or_else(|| {
            println!("Error: 'done' command requires a task ID.");
            print_usage();
            process::exit(1);
        });

        let id = id_str.parse::<usize>().unwrap_or_else(|_| {
            println!("Error: Invalid task ID. Please provide a number.");
            process::exit(1);
        });

        complete_task(context.tasks_mut(), id);
        context.transition_to(Box::new(SavingState {
            message: "Task marked as complete.".to_string(),
        }));
        context.execute(args);
    }

    fn name(&self) -> &str {
        "Complete"
    }
}

/// State for saving tasks to disk
struct SavingState {
    message: String,
}

impl AppState for SavingState {
    fn handle(&self, context: &mut AppContext, _args: &[String]) {
        if let Err(e) = save_tasks(context.tasks_mut()) {
            println!("Error saving tasks: {}", e);
            process::exit(1);
        }
        println!("{}", self.message);
        context.transition_to(Box::new(CompletedState));
    }

    fn name(&self) -> &str {
        "Saving"
    }
}

/// Completed tasks state
struct CompletedState;

impl AppState for CompletedState {
    fn handle(&self, _context: &mut AppContext, _args: &[String]) {
        // Terminal state - nothing to do
    }

    fn name(&self) -> &str {
        "Completed"
    }
}

// --- MAIN FUNCTION ---
// This is the entry point of the application. Execution starts here.
fn main() {
    let tasks = load_tasks().unwrap_or_else(|err| {
        println!(
            "Could not load task from file: {}. Starting with an empty list.",
            err
        );
        Vec::new() // `Vec::new()`creates a new, empty vector.
    });

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Create the application context with initial state
    let mut app = AppContext::new(tasks);

    // Execute the state machine
    app.execute(&args);

}

// Helper functions
/// Prints the list of tasks to the console.
/// This function BORROWS an immutable reference (`&Vec<Task>`) to the tasks.
/// It doesn't need to change the data, only read it.
fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks yet! Add one with the 'add' command.");
    } else {
        println!("--- Your Tasks ---");

        // `iter().enumerate()` gives both the index (i) and the item.
        for(i, task) in tasks.iter().enumerate() {
            let status  = if task.completed { "[x]" } else { "[ ]" };

            // Add 1 to the index `i` to create a user-friendly 1-based ID.
            println!("{} {}: {}", i + 1, status, task.description);
        }
    }
}

/// Adds a new task to the list.
/// This function BORROWS a mutable references because it needs to modify the vector by adding a new element.
/// The `description` parameter takes OWNERSHIP of the String value passed to it.
fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_task = Task {
        description: description, // The owned String is moved into the struct.
        completed: false,
    };
    tasks.push(new_task); // The `new_tasks` is moved into the vector.
}

/// Marks a task as complete by its 1-based ID.
/// This also BORROWS a mutable reference to change a task's status.
fn complete_task(tasks: &mut Vec<Task>, id: usize) {
    // Convert the user's 1-based ID to a 0-bazed vector index
    let index = id - 1;

    // `get_mut(index)` returns an `Option<&mut Task>`.
    // And it gives a mutable access to the element if it exists.
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
    } else {
        // If `get_mut` returns `None`, the ID was out of bounds.
        println!("Error: No task found with ID {}.", id);
    }
}

/// Loads the list of tasks from "tasks.json".
/// Returns a `Result` because file I/O can fail (eg. file not found, permission denied).
/// A `Result<Vec<Task>, std::io::Error>` means on success, get a `Vec<Task>`,
/// and on failure, get an I/O error object.
fn load_tasks() -> Result<Vec<Task>, std::io::Error> {

    // Open the file in read-only mode.
    // The `?` operator is crucial for Rust error handling. If `File::open` returns
    // an `Err`, the `?` will immediately return that error from the `load_tasks` function.
    // If it's `Ok`, it unwraps the value and continues.
    let file = File::open("tasks.json")?;

    let reader = BufReader::new(file);

    // Use serde_json to deserialize the JSON from the reader into a Vec<Task>.
    // This can also fail if the JSON is malformed, so use `?` again.
    let tasks = serde_json::from_reader(reader)?;

    // If everything succeeded, wrap the tasks vector in `Ok` to match the function's return type.
    Ok(tasks)
}

/// Saves the list of tasks to "tasks.json".
/// Also returns a `Result` to handle potential I/O errors.
/// The `()` is a "unit type" and signifies "no meaningful value" is returned on success.
fn save_tasks(tasks: &Vec<Task>) -> Result<(), std::io::Error> {
    // Create or ovewrite the file.
    let mut file = File::create("tasks.json")?;

    // Serialize the `tasks` vector into a JSON String. `.to_string_pretty()` makes it human-readable.
    let json_data = serde_json::to_string_pretty(tasks)?;

    // Write the JSON string to the file.
    file.write_all(json_data.as_bytes())?;

    // Return `Ok(())` on success.
    Ok(())
}

/// Prints the usage instructions for the application.
fn print_usage() {
    println!("--- Rusty Todos ---");
    println!("Usage: rusty-todos [COMMAND]");
    println!("\nCommands:");
    println!("  list              List all tasks");
    println!("  add “<desc>”      Add a new task");
    println!("  done <ID>         Complete a task by its ID");
}

// Lock file when states