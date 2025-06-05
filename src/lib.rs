#![allow(unused)]

pub mod todolist {
    use super::error::MyError;
    use ansi_term::Style;
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{self, BufReader, BufWriter};
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
        pub fn done(&mut self, index: usize) -> Result<(), MyError> {
            if index < 1 || index > self.list.len() {
                Err(MyError::IndexError {
                    message: String::from("Index out of bounds for the todo list"),
                })
            } else {
                self.list[index - 1].state = States::Done;

                Ok(())
            }
        }
        pub fn show(&self) -> Result<(), MyError> {
            let strike_through_style = Style::new().strikethrough();
            let mut index: usize = 1;
            // exit the function when no todo
            if self.list.is_empty() {
                println!("No todo for now. Remember to add new task!");
                return Ok(());
            }
            for todo in &self.list {
                let text = &format!("{}. {}", index, todo.term);
                match todo.state {
                    States::Undone => println!("{}", text),
                    States::Done => println!("{}", strike_through_style.paint(text)),
                }
                index += 1;
            }

            Ok(())
        }
        pub fn backup(&self, file_path: String) {
            todo!();
        }
        pub fn remove(&mut self, index: usize) {
            todo!();
        }
        pub fn edit(&mut self, index: usize, new_term: String) {
            todo!();
        }
        pub fn open(&mut self, file_path: String) {
            todo!();
        }
        pub fn read_json<P: AsRef<Path>>(&mut self, file_path: P) -> Result<(), MyError> {
            let file = File::open(&file_path)?;
            let reader = BufReader::new(file);
            self.list = serde_json::from_reader(reader)?;

            Ok(())
        }
        pub fn write_json<P: AsRef<Path>>(&self, file_path: P) -> Result<(), MyError> {
            let file = File::create(file_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &self.list)?;

            Ok(())
        }
    }
}

pub mod cli {
    use super::error::MyError;
    use super::todolist::{Todo, TodoList};
    use clap::{Parser, Subcommand};
    use std::io;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand, Debug)]
    pub enum Commands {
        Add { term: String },
        Done { index: usize },
        Show,
    }

    pub fn run(todolist: &mut TodoList, args: Args) -> Result<(), MyError> {
        match args.command {
            Some(Commands::Add { term }) => {
                todolist.add(Todo::new(term));
            }
            Some(Commands::Done { index }) => {
                todolist.done(index)?;
            }
            _ => (),
        }

        todolist.show()?;

        Ok(())
    }
}

pub mod error {
    use std::fmt;
    use std::fs::File;
    use std::io;

    use clap::builder::Str;

    #[derive(Debug)]
    pub enum MyError {
        IoError { message: String },
        JSONError { message: String },
        IndexError { message: String },
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MyError::IndexError { message } => write!(f, "{message}"),
                MyError::IoError { message } => write!(f, "{message}"),
                MyError::JSONError { message } => write!(f, "{message}"),
            }
        }
    }

    impl From<serde_json::Error> for MyError {
        fn from(error: serde_json::Error) -> MyError {
            use serde_json::error::Category;
            match error.classify() {
                Category::Io => MyError::IoError {
                    message: error.to_string(),
                },
                _ => MyError::JSONError {
                    message: error.to_string(),
                },
            }
        }
    }

    impl From<io::Error> for MyError {
        fn from(error: io::Error) -> MyError {
            MyError::IoError {
                message: error.to_string(),
            }
        }
    }
}
