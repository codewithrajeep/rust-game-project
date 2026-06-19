// io is a module that provides input/output functionality.
use std::io;

fn main() {
    // Welcome the user.
    println!("📝 Welcome to Rust To-Do List");
    // A Vector to store the to-do items.
    let mut tasks: Vec<String> = Vec::new();

    // Loop to continuously prompt the user for input.
    loop {
        // commands used to interact with the to-do list.
        println!("\nCommands: [add], [view], [remove], [quit]");
        // Ask the user to input a command
        println!("Enter the command:");
        // getting user input command
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        // trime the whitespace and match the command
        match command.trim() {
            // add new task
            "add" => {
                // asking user for the description of task
                println!("Enter task description");
                // getting user input for the task description
                let mut task = String::new();
                // reading user input from stdin
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read line");
                // trimming the whitespace from the task description and pushing it to the vector
                tasks.push(task.trim().to_string());
                // displaying added task message
                println!("Task added!");
            }
            // viewing list of tasks
            "view" => {
                // check if tasks is empty or not
                if tasks.is_empty() {
                    println!("No tasks yet.")
                } else {
                    // displaying the list of tasks
                    println!("--- Your Tasks ---");
                    // loop through tasks and print each one with its index
                    // iter is a method that returns an iterator over the collection
                    // enumerate is a method that returns a tuple of the index and the value
                    for (index, task) in tasks.iter().enumerate() {
                        // index + 1 is the task number, task is the task description
                        println!("{}. {}", index + 1, task)
                    }
                }
            }
            // removing a task
            "remove" => {
                // check if tasks is empty or not
                if tasks.is_empty() {
                    println!("No tasks to remove.");
                    // continue the program loop
                    continue;
                }
                // asking user for the task number to remove
                println!("Enter the number of the task to remove:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                // convert input to a usize(unsigned integer)
                // remove the whitespace of input as well
                match input.trim().parse::<usize>() {
                    Ok(index) => {
                        // conver 1-based index to 0-based index
                        // means, computers store data in 0-based index and we have store data in 1-based index.
                        // i.e, if the task index is 5, the 0-based index is 4. So, we have to get the computer based by subtracting by 1
                        let task_index = index - 1;
                        // check if the task index is valid or not
                        if task_index < tasks.len() {
                            // remove the task at the given index
                            let removed = tasks.remove(task_index);
                            // print the removed task
                            println!("Removed: '{}'", removed)
                        } else {
                            // user inputs an invalid index
                            println!(
                                "Invalid index. Please enter a number between 1 and {}",
                                tasks.len()
                            )
                        }
                    }
                    // accepts any other input that is not a valid index
                    Err(_) => {
                        println!(
                            "Invalid input. Please enter a number between 1 and {}",
                            tasks.len()
                        )
                    }
                }
            }
            // quitting the program
            "quit" => {
                println!("Goodbye!");
                // break out of the loop
                break;
            }
            // if user input the invalid command
            _ => {
                println!("Unknown Command. Try 'add', 'view' or 'quit' ")
            }
        }
    }
}
