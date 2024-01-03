mod todo_ops;
mod menu_io;
mod models;
mod file_io;

use console::{Color, style};

use std::process::exit;
use crate::models::TodoItem;
use crate::todo_ops::mark_todo_done;

fn main() {

    let todos: (Vec<TodoItem>, Vec<TodoItem>) = file_io::load_or_create_file();

    println!("\nWelcome to {}", style("rusty-todos").bold().blue().bg(Color::Red));

    let mut pending_todos: Vec<TodoItem> = todos.0;
    let mut completed_todos: Vec<TodoItem> = todos.1;

    loop {
        let option = menu_io::choose_from_main_menu();

        match option {
            0 => {
                //save_state(pending_todos, completed_todos);
                exit(0)
            },
            1 => {
                match pending_todos.len() {
                    0 => println!("No todos."),
                    _ => {
                        show_list_of_todos_with_todo_options(&mut pending_todos, &mut completed_todos);
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
            _ => {
                println!("Failed to choose. try again");
                break;
            }
        }
    }
}

fn show_list_of_todos_with_todo_options(mut pending_todos: &mut Vec<TodoItem>, mut completed_todos: &mut Vec<TodoItem>) {
    println!("Here are your pending todos:");
    todo_ops::list_todos(&pending_todos);

    println!("Type todo option number to open a todo.");
    let chosen_todo_index = menu_io::choose_option() - 1;

    let current_todo = &pending_todos[chosen_todo_index];
    todo_ops::open_a_todo(current_todo);

    operate_on_a_todo(
        chosen_todo_index,
        &mut pending_todos, &mut completed_todos);
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
                pending_todos[chosen_todo_index] = edited_todo;
                continue;
            }
            2 => { // complete
                mark_todo_done(chosen_todo_index, pending_todos, completed_todos);
                break;
            }
            3 => { // remove
                pending_todos.remove(chosen_todo_index);
                break;
            }
            num => {
                println!("Chose: {}", num);
                break;
            }
        }
    };
}


