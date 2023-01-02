use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("USAGE: todo COMMAND [ARGS]");
        println!("Commands:");
        println!(" add 'TODO item'");
        println!(" list");
        println!(" done INDEX");
        return;
    }

    let command = &args[1];
    let mut todo_list = read_todo_list();

    if command == "add" {
        let todo_item = &args[2..].join(" ");
        add_todo_item(&mut todo_list, todo_item);
        write_todo_list(&todo_list);
    } else if command == "list" {
        list_todo_items(&todo_list);
    } else if command == "done" {
        let index: usize = args[2].parse().unwrap();
        mark_todo_item_as_done(&mut todo_list, index);
        write_todo_list(&todo_list);
    } else {
        println!("Unrecognized command: {}", command);
    }
}

fn read_todo_list() -> Vec<String> {
    let contents = fs::read_to_string("todo.txt")
        .expect("Failed to read todo.txt");
    
    contents.lines().map(String::from).collect()
}

fn write_todo_list(todo_list: &Vec<String>) {
    let contents = todo_list.join("\n");
    fs::write("todo.txt", contents).expect("Failed to wirte todo.txt");
}

fn add_todo_item(todo_list: &mut Vec<String>, todo_item: &str) {
    todo_list.push(todo_item.to_string());
    println!("Todo item added: {}", todo_item);
}

fn list_todo_items(todo_list: &Vec<String>) {
    println!("Todo list:");
    for (index, todo_item) in todo_list.iter().enumerate() {
        println!("{}. {}", index + 1, todo_item);
    }
}

fn mark_todo_item_as_done(todo_list: &mut Vec<String>, index: usize) {
    if index >= todo_list.len() {
        println!("Invalid index: {}", index);
    }

    let todo_item = todo_list[index].clone();
    println!("Todo item marked as done: {}", todo_item);
    todo_list.remove(index);
}