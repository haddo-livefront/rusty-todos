use std::env;
use std::process;
use rusty_todo::{AppContext, load_tasks, Command, CommandResult};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = match parse_args(&args) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("{}", e);
            print_usage();
            process::exit(1);
        }
    };

    let tasks = load_tasks().unwrap_or_else(|err| {
        println!(
            "Could not load task from file: {}. Starting with an empty list.",
            err
        );
        Vec::new()
    });

    let mut app = AppContext::new(tasks);
    match app.execute(command) {
        Ok(result) => match result {
            CommandResult::Message(msg) => println!("{}", msg),
            CommandResult::Tasks(tasks) => {
                if tasks.is_empty() {
                    println!("No tasks yet! Add one with the 'add' command.");
                } else {
                    println!("--- Your Tasks ---");
                    for(i, task) in tasks.iter().enumerate() {
                        let status  = if task.completed { "[x]" } else { "[ ]" };
                        println!("{} {}: {}", i + 1, status, task.description);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}


fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err("Invalid command.".to_string());
    }

    let command_str = &args[1];
    match command_str.as_str() {
        "list" => Ok(Command::List),
        "version" | "-v" | "--version" => Ok(Command::Version),
        "add" => {
            let description = args.get(2).cloned().ok_or("Error: 'add' command requires a description.".to_string())?;
            Ok(Command::Add(description))
        },
        "done" => {
            let id = args.get(2).cloned().ok_or("Error: 'done' command requires a task ID.".to_string())?;
            Ok(Command::Complete(id))
        },
        "undone" => {
            let id = args.get(2).cloned().ok_or("Error: 'undone' command requires a task ID.".to_string())?;
            Ok(Command::Uncomplete(id))
        },
        "delete" => {
            let id = args.get(2).cloned().ok_or("Error: 'delete' command requires a task ID.".to_string())?;
            Ok(Command::Delete(id))
        },
        "edit" => {
            let id = args.get(2).cloned().ok_or("Error: 'edit' command requires a task ID.".to_string())?;
            let description = args.get(3).cloned().ok_or("Error: 'edit' command requires a new description.".to_string())?;
            Ok(Command::Edit(id, description))
        },
        _ => Err(format!("Error: Unknown command '{}'", command_str)),
    }
}

fn print_usage() {
    println!("--- Rusty Todos ---");
    println!("Usage: rusty-todos [COMMAND]");
    println!("\nCommands:");
    println!("  list              List all tasks");
    println!("  add “<desc>”      Add a new task");
    println!("  done <ID>         Complete a task by its ID");
    println!("  undone <ID>       Mark a task as incomplete by its ID");
    println!("  delete <ID>       Delete a task by its ID");
    println!("  edit <ID> <desc>  Edit a task by its ID");
}

