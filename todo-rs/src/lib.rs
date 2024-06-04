
use chrono::prelude::*;
use clap::Parser;
use std::error::Error;
use std::str::FromStr;

use commands::add::add_task;
use commands::list::list_tasks;

mod commands {
    pub mod add;
    pub mod list;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub add: bool,
    #[arg(short, long)]
    pub list: bool,
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

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ToDo" => Ok(Status::ToDo),
            "Done" => Ok(Status::Done),
            _ => Err(()),
        }
    }
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
        Config { add: true, .. } => add_task()?,
        Config { list: true, .. } => list_tasks()?,
        _ => { println!("No command provided"); }
    };

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