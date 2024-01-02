use console::Term;

pub fn choose_from_main_menu() -> usize {
    println!("\n_____________________");
    println!("Main menu:");

    println!("0. Exit");
    println!("1. List todos");
    println!("2. Add a Todo");
    let result = choose_option();
    clearscreen::clear().expect("failed to clear screen");

    result
}

pub fn choose_option() -> usize {
    let term = Term::stdout();
    let ch = term.read_char().expect("TODO: panic message");
    ch as usize - '0' as usize
}

pub fn show_todo_menu() {
    println!("0. Exit todo");
    println!("1. Edit");
    println!("2. Mark it done");
    println!("3. Delete");
}