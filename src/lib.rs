#![allow(unused)]

// todo object to act on
pub mod todolist {
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use std::path::Path;

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct Todo {
        term: String,
        category: String,
        proirity: i32,
        state: States,
    }

    #[derive(Deserialize, Serialize)]
    pub enum States {
        Undone,
        Done,
    }

    #[derive(Deserialize, Serialize)]
    pub struct TodoList {
        pub todolist: Vec<Todo>,
    }

    impl Todo {
        pub fn new(term: &str, category: &str, proirity: i32) -> Result<Todo, Box<dyn Error>> {
            Ok(Todo {
                term: String::from(term),
                category: String::from(term),
                proirity,
                state: States::Undone,
            })
        }
    }

    impl TodoList {
        pub fn new(todolist: Vec<Todo>) -> Result<TodoList, Box<dyn Error>> {
            Ok(TodoList { todolist })
        }
        pub fn add(&mut self, todo: Todo) -> Result<(), Box<dyn Error>> {
            self.todolist.push(todo);
            Ok(())
        }
        pub fn remove(&mut self, index: usize) {
            todo!();
        }
        pub fn edit(&mut self, index: usize, new_todo: Todo) {
            todo!();
        }
        pub fn done(&mut self, index: usize) {
            todo!();
        }
        pub fn read_from_json<P: AsRef<Path>>(file_path: P) -> Result<TodoList, Box<dyn Error>> {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            let todo_list = serde_json::from_reader(reader)?;
            Ok(todo_list)
        }
        pub fn write_json<P: AsRef<Path>>(&self, file_path: P) -> Result<(), Box<dyn Error>> {
            let file = File::create(file_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, self)?;
            Ok(())
        }
    }
}

mod cli {
    fn parser(args: Vec<String>) -> crate::todolist::Todo {
        todo!();
    }
}
