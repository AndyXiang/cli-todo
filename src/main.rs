#![allow(unused)]
use cli_todo::todolist::{States, Todo, TodoList};

fn main() {
    let mut todo_list = TodoList::new(vec![]).unwrap();

    todo_list.add(Todo::new("Buy milk", "Daily", 2).unwrap());
    todo_list.add(Todo::new("Sleep", "Daily", 3).unwrap());

    todo_list.write_json("todo.json");
}
