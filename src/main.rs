#![allow(unused)]

use std::process;

use clap::Parser;
use cli_todo::cli::{Args, run};
use cli_todo::error::MyError;
use cli_todo::todolist::{Todo, TodoList, init};

fn main() {
    let mut todo_list: TodoList = TodoList { list: vec![] };

    let file_path = init().unwrap_or_else(|err| {
        println!("Problem init the todo.json file: {}", err);
        process::exit(1);
    });

    todo_list.read_json(&file_path).unwrap_or_else(|err| {
        println!("Problem reading todo history from json: {}", err);
        process::exit(1);
    });

    let args = Args::parse();

    run(&mut todo_list, args).unwrap_or_else(|err| {
        println!("Problem dealing with commands: {}", err);
        process::exit(1);
    });

    todo_list.write_json(&file_path).unwrap_or_else(|err| {
        println!("Problem writing todo history to json: {}", err);
        process::exit(1);
    });
}
