use std::fs::File;
use std::io;
use std::io::{ErrorKind, Write};
use serde_json::json;
use crate::models::{TodoItem, TodoStatus};

pub fn load_or_create_file() -> (Vec<TodoItem>, Vec<TodoItem>) {
    let mut pending_todos: Vec<TodoItem> = Vec::new();
    let mut completed_todos: Vec<TodoItem> = Vec::new();

    match File::open("todos.json") {
        Ok(file) => {
            let loaded_todos: Vec<TodoItem> = serde_json::from_reader(io::BufReader::new(file)).expect("Failed to load file");

            for val_item in loaded_todos {
                if val_item.status == TodoStatus::PENDING {
                    pending_todos.push(TodoItem {
                        content: val_item.content,
                        status: TodoStatus::PENDING,
                    });
                } else {
                    completed_todos.push(TodoItem {
                        content: val_item.content,
                        status: TodoStatus::DONE,
                    });
                }
            }
        }
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                File::create("todos.json").unwrap();
            }
        }
    }

    (pending_todos, completed_todos)
}

// pub fn save_state(pending: Vec<TodoItem>, done: Vec<TodoItem>) {
//     let all = pending.into_iter().chain(done.into_iter()).collect();
//
//     match File::open("todos.json") {
//         Ok(mut file) => {
//             let content = serde_json::to_string(all).unwrap();
//             file.write_all(content.as_bytes()).expect("TODO: panic message");
//             file.flush().expect("TODO: panic message");
//             Ok(())
//         }
//         Err(err) => {
//             if err.kind() == ErrorKind::NotFound {
//                 File::create("todos.json").unwrap();
//                 save_state(pending, done);
//             }
//         }
//     };
// }