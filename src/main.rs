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

// --- MAIN FUNCTION ---
// This is the entry point of the application. Execution starts here.
fn main() {
    let mut tasks = load_tasks().unwrap_or_else(|err| {
        println!(
            "Could not load task from file: {}. Starting with an empty list.",
            err
        );
        Vec::new() // `Vec::new()`creates a new, empty vector.
    });

    let args: Vec<String> = env::args().collect();

    let command = args.get(1).unwrap_or_else(|| {
        print_usage();
        process::exit(1);
    });

    // Use `match`to execute the corect logic based on the command string.
    match command.as_str() {
        "list" => list_tasks(&tasks), // Pass an immutable reference to the tasks.

        "add" => {
            // The task description should be the argument at index 2.
            let description = args.get(2).cloned().unwrap_or_else(|| {
                println!("Error: 'add' command requires a description.");
                print_usage();
                process::exit(1);
            });
            add_task(& mut tasks, description); // Pass a mutable reference to modify the list.
            save_and_confirm(&tasks, "Task added.");
        }

        "done" => {
            let id_str = args.get(2).unwrap_or_else(|| {
                println!("Error: Invalid task ID. Please provide a number");
                process::exit(1);
            });

            // Parse the ID string into a number. `parse()` returns a `Result`.
            // Handle the error case where the input isn't a valid number.
            let id = id_str.parse::<usize>().unwrap_or_else(|_| {
                println!("Error: Invalid task ID. Please provide a number.");
                process::exit(1);
            });

            complete_task(&mut tasks, id);  // Pass a mutable reference to modify the task.
            save_and_confirm(&tasks, "Task marked as complete.");
        }

        _=> {
            println!("Error Unknown command '{}'", command);
            print_usage();
            process::exit(1);
        }
    }
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

/// A small utility function to save tasks and print a confirmation message.
fn save_and_confirm(tasks: &Vec<Task>, message: &str) {
    // `if let Err(e) = ...` is a clean way to handle a `Result` when you only
    // care about the error case.
    if let Err(e) = save_tasks(tasks) {
        println!("Error saving tasks: {}", e);
        process::exit(1);
    }
    println!("{}", message);
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