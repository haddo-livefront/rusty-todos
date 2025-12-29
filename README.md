# rusty-todos
Hacking with Rust

## How to Run it

Run the following commands at the root of the project:

```sh
# Add a new task
$ cargo run -- add "Buy groceries"

# Add another task
$ cargo run -- add "Learn Rust ownership concept"

# List all tasks
$ cargo run -- list

# Mark the task with index 1 as done

$ cargo run -- done 1
```

## State Machine Pattern

### AppContext - Context class
1. Holds the current and overal application state(tasks)
2. Provides methods to transition between states
3. Delegates behaviour to the current state


### AppState Trait - State Interface
1. Defines the contracts that all states must implement

### Concrete States
IdleState - Initial state routing to the appropriate operation states
ListState - Displays all tasks
AddState - Adds a new task
CompleteState - Terminal state indicating successful completion

## State Flow
Idle -> [List/Add/Complete] -> Saving -> Completed
