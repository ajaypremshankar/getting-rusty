mod todo_ops;
mod menu_io;
mod models;

use console::{Color, style};

use std::process::exit;
use crate::models::TodoItem;

fn main() {
    println!("\nWelcome to {}", style("rusty-todos").bold().blue().bg(Color::Red));

    let mut pending_todos: Vec<TodoItem> = Vec::new();
    let mut completed_todos: Vec<TodoItem> = Vec::new();

    loop {
        let option = menu_io::choose_from_main_menu();

        match option {
            0 => exit(0),
            1 => {
                match pending_todos.len() {
                    0 => println!("No todos."),
                    _ => {
                        println!("Here are your pending todos:");
                        todo_ops::list_todos(&pending_todos);

                        println!("Type todo option number to open a todo.");
                        let chosen_todo_index = menu_io::choose_option();

                        let current_todo = &pending_todos[chosen_todo_index - 1];
                        todo_ops::open_a_todo(current_todo);

                        operate_on_a_todo(
                            chosen_todo_index,
                            &mut pending_todos, &mut completed_todos);
                        continue;
                    }
                }
            }
            2 => {
                let todo = todo_ops::get_todo_from_user();
                pending_todos.push(todo);
                println!("Todo added successfully");
                continue;
            }
            3 => {
                println!("Here are your current pending todos:");
                todo_ops::list_todos(&pending_todos);
                println!("Type todo option number to mark it done.");

                let chosen_item = menu_io::choose_option();
                match chosen_item {
                    num => {
                        let todo = pending_todos.remove(num - 1);
                        completed_todos.push(todo);
                    }
                }

                println!("Completed Todos");
                todo_ops::list_todos(&completed_todos);

                continue;
            }
            _ => {
                println!("Failed to choose. try again");
                continue;
            }
        }
    }
}

fn operate_on_a_todo(
    chosen_todo_index: usize,
    pending_todos: &mut Vec<TodoItem>,
    completed_todos: &mut Vec<TodoItem>) {
    let current_todo = &pending_todos[chosen_todo_index].clone();

    loop {
        menu_io::show_todo_menu();
        let chosen_item_on_todo_menu = menu_io::choose_option();

        match chosen_item_on_todo_menu {
            0 => { // exit
                break;
            }
            1 => { // edit
                let edited_todo = todo_ops::edit_todo(current_todo);
                pending_todos[chosen_todo_index - 1] = edited_todo;
                continue;
            }
            2 => { // complete
                let todo = pending_todos.remove(chosen_todo_index - 1);
                completed_todos.push(todo);
                break;
            }
            3 => { // remove
                pending_todos.remove(chosen_todo_index - 1);
                break;
            }
            num => {
                println!("Chose: {}", num);
                break;
            }
        }
    };
}


