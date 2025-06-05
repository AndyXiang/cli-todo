#![allow(unused)]

use clap::Parser;
use cli_todo::cli::{Args, run};
use cli_todo::todolist::{Todo, TodoList};

fn main() {
    let mut todo_list: TodoList = TodoList { list: vec![] };

    todo_list.read_json(String::from("todo.json"));

    let args = Args::parse();

    run(&mut todo_list, args);

    todo_list.write_json(String::from("todo.json"));
}
