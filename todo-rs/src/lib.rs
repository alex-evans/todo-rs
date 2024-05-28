
use chrono::prelude::*;
use clap::Parser;
use std::error::Error;
use std::io;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub add: bool,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let args = Config::parse();
        Ok(args)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Status {
    ToDo,
    Done,
}

struct TodoTask {
    title: String,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TodoTask {
    fn new(title: String, description: String) -> TodoTask {
        let utc_now: DateTime<Utc> = Utc::now();

        TodoTask {
            title,
            description,
            status: Status::ToDo,
            created_at: utc_now,
            updated_at: utc_now,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config { add: true, .. } => add_task(config)?,
        _ => { println!("No command provided"); }
    };

    Ok(())
}

fn add_task(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Adding Task");
    println!("Please enter the title of the task: ");
    let mut title = String::new();

    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

    let title: String = match title.trim().parse() {
        Ok(title) => title,
        Err(_) => {
            println!("Please enter a valid title");
            return Ok(());
        }
    };

    println!("Please enter the description of the task: ");
    let mut description = String::new();

    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    let description: String = match description.trim().parse() {
        Ok(description) => description,
        Err(_) => {
            println!("Please enter a valid description");
            return Ok(());
        }
    };

    let task = TodoTask::new(title, description);

    save_task(task)?;
    println!("Task added successfully");

    Ok(())
}

fn save_task(task: TodoTask) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("tasks.txt")?;

    writeln!(file, "{}, {}, {:?}, {}, {}", task.title, task.description, task.status, task.created_at, task.updated_at)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_task_new() {
        let title = String::from("Task 1");
        let description = String::from("Description 1");
        let task = TodoTask::new(title.clone(), description.clone());

        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert_eq!(task.status, Status::ToDo);
        assert_eq!(task.created_at, task.updated_at);
    }

    #[test]
    fn test_save_task() {
        let task = TodoTask {
            title: String::from("Task 2"),
            description: String::from("Description 2"),
            status: Status::ToDo,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = save_task(task);

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_no_add() {
        let config = Config { add: false };

        let result = run(config);

        assert!(result.is_ok());
    }
}