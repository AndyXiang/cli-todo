#![allow(unused)]

pub mod todolist {
    use ansi_term::Style;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use std::path::Path;

    #[derive(Deserialize, Serialize)]
    pub enum States {
        Undone,
        Done,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Todo {
        pub term: String,
        pub state: States,
    }

    impl Todo {
        pub fn new(term: String) -> Todo {
            Todo {
                term,
                state: States::Undone,
            }
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct TodoList {
        pub list: Vec<Todo>,
    }

    impl TodoList {
        pub fn add(&mut self, todo: Todo) {
            self.list.push(todo);
        }
        pub fn done(&mut self, index: usize) {
            self.list[index - 1].state = States::Done;
        }
        pub fn show(&self) {
            let strike_through_style = Style::new().strikethrough();
            let mut index: usize = 1;
            for todo in &self.list {
                let text = &format!("{}. {}", index, todo.term);
                match todo.state {
                    States::Undone => println!("{}", text),
                    States::Done => println!("{}", strike_through_style.paint(text)),
                }
                index += 1;
            }
        }
        pub fn read_json(&mut self, file_path: String) {
            let file = File::open(file_path).unwrap();
            let reader = BufReader::new(file);
            self.list = serde_json::from_reader(reader).unwrap();
        }
        pub fn write_json(&self, file_path: String) {
            let file = File::create(file_path).unwrap();
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &self.list);
        }
    }
}

pub mod cli {
    use super::todolist::{Todo, TodoList};
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        #[arg(short, long, num_args(0..))]
        add: Option<String>,

        #[arg(short, long, num_args(0..))]
        done: Option<usize>,

        #[arg(short, long, num_args(0..))]
        show: Option<String>,
    }

    pub fn run(todolist: &mut TodoList, args: Args) {
        if let Some(term) = args.add {
            todolist.add(Todo::new(term.clone()));
        }
        if let Some(index) = args.done {
            todolist.done(index);
        }
        todolist.show();
    }
}
