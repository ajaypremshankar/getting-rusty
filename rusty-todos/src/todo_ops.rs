use std::io;
use rustyline::DefaultEditor;
use crate::models::{TodoItem, TodoStatus};

pub fn get_todo_from_user() -> TodoItem {
    let mut option = String::new();
    println!("Enter todo and press enter.");

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    TodoItem {
        content: option.to_string(),
        status: TodoStatus::PENDING
    }
}

pub fn edit_todo(todo: &TodoItem) -> TodoItem {
    let mut rl = DefaultEditor::new().unwrap();
    let default_username = todo.content.as_str();
    let input = rl.readline_with_initial("Edit : ", (default_username, "")).unwrap();

    TodoItem {
        content: input,
        status: TodoStatus::PENDING
    }
}

pub fn list_todos(todos: &Vec<TodoItem>) {
    for (i, todo) in todos.iter().enumerate() {
        println!("{}. {}", i + 1, todo.content);
    }
}

pub fn open_a_todo(todo: &TodoItem) {
    println!("__________________");
    println!("Current Todo: {}", todo.content);
}

pub fn mark_todo_done(chosen_todo_index: usize,
                      pending_todos: &mut Vec<TodoItem>,
                      completed_todos: &mut Vec<TodoItem>) {
    let todo = pending_todos.remove(chosen_todo_index - 1);
    completed_todos.push(todo);
}