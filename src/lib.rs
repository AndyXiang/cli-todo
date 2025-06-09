#![allow(unused)]

const DEFAULT_PATH: &str = "~/.cache/todo/todo.json";

pub mod todolist {
    use super::error::MyError;
    use ansi_term::Style;
    use clap::builder::Str;
    use dirs::home_dir;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fs;
    use std::fs::File;
    use std::io::{self, BufReader, BufWriter};
    use std::path::{Path, PathBuf};

    #[derive(Deserialize, Serialize)]
    pub enum States {
        Undone,
        Done,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Todo {
        pub term: String,
        pub state: States,
        pub category: String,
    }

    impl Todo {
        pub fn new(term: String, category: String) -> Todo {
            Todo {
                term,
                state: States::Undone,
                category,
            }
        }
    }

    impl Clone for Todo {
        fn clone(&self) -> Self {
            Self {
                term: self.term.clone(),
                state: match self.state {
                    States::Undone => States::Undone,
                    States::Done => States::Done,
                },
                category: self.category.clone(),
            }
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct TodoList {
        pub list: Vec<Todo>,
    }

    impl TodoList {
        pub fn new(list: Vec<Todo>) -> TodoList {
            TodoList { list }
        }
        pub fn add(&mut self, todo: Todo) {
            self.list.push(todo);
        }
        pub fn done(&mut self, index: usize) -> Result<(), MyError> {
            if index >= self.list.len() {
                Err(MyError::IndexError {
                    message: String::from("Index out of bounds for the todo list"),
                })
            } else {
                self.list[index].state = States::Done;

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
        pub fn group_by_category(&self) -> HashMap<&str, TodoList> {
            let mut hashmap_by_category: HashMap<&str, TodoList> = HashMap::new();
            for todo in &self.list {
                let category = &todo.category;
                let todo_clone = todo.clone();
                hashmap_by_category
                    .entry(&todo.category)
                    .or_insert(TodoList::new(vec![]))
                    .list
                    .push(todo_clone);
            }

            hashmap_by_category
        }
        pub fn sort_by_index(&mut self) -> Result<(), MyError> {
            let mut i = 0;
            while i < self.list.len() {
                match self.list[i].state {
                    States::Undone => {
                        // if the todo is undone, leave it and examine next one
                        i += 1;
                        continue;
                    }
                    States::Done => {
                        // move the done todo to the tail
                        // no need to increase i since the the next one is advanced
                        let done_todo = self.list.remove(i);
                        self.list.push(done_todo);
                    }
                }
            }
            Ok(())
        }
        pub fn remove_by_index(&mut self, index: usize) -> Result<(), MyError> {
            if index >= self.list.len() {
                Err(MyError::IndexError {
                    message: String::from("Index out of bounds for the todo list"),
                })
            } else {
                self.list.remove(index);
                Ok(())
            }
        }
        pub fn edit(&mut self, index: usize, new_term: String) -> Result<(), MyError> {
            if index >= self.list.len() {
                Err(MyError::IndexError {
                    message: String::from("Index out of bounds for the todo list"),
                })
            } else {
                self.list[index].term = new_term;
                Ok(())
            }
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

    pub fn init() -> Result<PathBuf, MyError> {
        let home = home_dir().ok_or_else(|| MyError::IoError {
            message: String::from("Cant identify home directory."),
        })?;

        let folder_path = home.join(".cache/todo");
        if !folder_path.exists() {
            fs::create_dir_all(&folder_path)?;
        }

        let file_path = folder_path.join("todo.json");
        if !file_path.exists() {
            let init_list = TodoList { list: vec![] };
            init_list.write_json(&file_path)?;
        }

        Ok(file_path)
    }
}

pub mod cli {
    use super::error::MyError;
    use super::todolist::{States, Todo, TodoList, init};
    use clap::{Parser, Subcommand};
    use std::collections::HashMap;
    use std::io;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    pub struct Args {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }

    #[derive(Subcommand, Debug)]
    pub enum Commands {
        /// Add new todo item
        Add {
            term: Vec<String>,

            #[arg(short, long, default_value = "default")]
            category: String,
        },
        /// Done todos
        Done { index: Vec<usize> },
        /// Remove todos
        Remove { index: Vec<usize> },
        /// Edit the content of one todo
        Edit { index: usize, term: Vec<String> },
        /// Clear all todo
        Clear {
            #[arg(short, long, default_value = "false")]
            all: bool,
        },
        ///Sort the todo list and put all done todo to tail
        Sort {
            #[arg(short, long, default_value = "true")]
            by_index: bool,
        },
        /// Init a todo list
        Init,
        /// Show current todo list
        Show {
            #[arg(short, long, default_value = "false")]
            category: bool,
        },
    }

    pub fn run(todolist: &mut TodoList, args: Args) -> Result<(), MyError> {
        match args.command {
            Some(Commands::Add { term, category }) => {
                todolist.add(Todo::new(term.join(" "), category));
                todolist.show()?;
            }
            Some(Commands::Done { index }) => {
                // user input start from 1, the list index start from 0
                for i in index {
                    todolist.done(i - 1)?;
                }
                todolist.show()?;
            }
            Some(Commands::Remove { index }) => {
                for i in index {
                    todolist.remove_by_index(i - 1)?;
                }
                todolist.show()?;
            }
            Some(Commands::Edit { index, term }) => {
                todolist.edit(index - 1, term.join(" "));
                todolist.show()?;
            }
            Some(Commands::Init) => {
                init();
                todolist.show()?;
            }
            Some(Commands::Clear { all }) => {
                if all {
                    todolist.list.clear();
                    todolist.show()?;
                } else {
                    todolist.list.retain(|x| match x.state {
                        States::Undone => true,
                        States::Done => false,
                    });
                    todolist.show()?;
                }
            }
            Some(Commands::Sort { by_index }) => {
                if by_index {
                    todolist.sort_by_index();
                }
                todolist.show()?;
            }
            Some(Commands::Show { category }) => {
                if category {
                    let hashmap_by_categroy = todolist.group_by_category();
                    for (k, v) in &hashmap_by_categroy {
                        println!("\n{}:", k);
                        v.show();
                    }
                }
            }
            _ => {
                todolist.show()?;
            }
        }
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
