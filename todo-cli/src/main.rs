use std::io;

fn main() {
    let mut todos = Vec::new();

    loop {
        println!("Please enter a command (add, list, quit):");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        
        let command = command.trim();

        if command == "quit" {
            break;
        } else if command == "add" {
            println!("Enter a todo item:");
            let mut todo = String::new();

            io::stdin()
                .read_line(&mut todo)
                .expect("Failed to read line");

            let todo = todo.trim();
            todos.push(todo.to_string());
        } else if command == "list" {
            for (index, todo) in todos.iter().enumerate() {
                println!("{}: {}", index + 1, todo);
            }
        } else {
            println!("Invalid command");
        }
    }

    
}