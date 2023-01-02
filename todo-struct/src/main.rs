use std::io;


struct Task {
    name: String,
    completed: bool
}
fn main() {
    let mut tasks = Vec::new();

    loop {
        println!("Please choose an option:");
        println!("1. Add a task");
        println!("2. Mark a task as completed");
        println!("3. List all tasks");
        println!("4. Quit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => add_task(&mut tasks),
            2 => mark_task_as_completed(&mut tasks),
            3 => list_tasks(&tasks),
            4 => break,
            _ => println!("Invalid option"),
        }
        
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    println!("Enter the name of the task:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    let name = name.trim();

    let task = Task {
        name: name.to_string(),
        completed: false,
    };

    tasks.push(task);
    println!("Task Added!");
}

fn mark_task_as_completed(tasks: &mut Vec<Task>) {
    println!("Enter the index of the task to mark as completed");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if index >= tasks.len() {
        print!("Invalid index");
        return;
    }

    let task = &mut tasks[index];
    task.completed = true;
}

fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        println!("{} {} ({})", index, task.name, task.completed);
    }
}
