
use chrono::prelude::*;
use clap::Parser;
use std::io::Error as IoError;
use std::io::ErrorKind;
use std::error::Error;
use std::str::FromStr;

use commands::add::add_task;
use commands::complete::complete_task;
use commands::delete::delete_task;
use commands::edit::edit_task;
use commands::list::list_tasks;

mod commands {
    pub mod add;
    pub mod list;
    pub mod complete;
    pub mod edit;
    pub mod delete;
}

mod helpers {
    pub mod helpers;
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub add: bool,
    #[arg(short, long)]
    pub list: bool,
    #[arg(short, long)]
    pub complete: bool,
    #[arg(short, long)]
    pub edit: bool,
    #[arg(short, long)]
    pub delete: bool,
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
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ToDo" => Ok(Status::ToDo),
            "Done" => Ok(Status::Done),
            _ => Err(IoError::new(ErrorKind::InvalidInput, "Invalid status")),
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

    fn from_csv(line: String) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 5 {
            return Err("Invalid CSV format".into());
        }
        let status = parts[2].trim();
        let status_enum = Status::from_str(status)?;
        Ok(TodoTask {
            title: parts[0].to_string(),
            description: parts[1].to_string(),
            status: status_enum,
            created_at: parts[3].parse()?,
            updated_at: parts[4].parse()?,
        })
    }

    fn to_csv(&self) -> String {
        format!("{}, {}, {:?}, {}, {}", self.title, self.description, self.status, self.created_at, self.updated_at)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config {
        Config { add: true, .. } => add_task()?,
        Config { list: true, .. } => list_tasks()?,
        Config { complete: true, .. } =>  complete_task()?,
        Config { edit: true, .. } => edit_task()?,
        Config { delete: true, .. } => delete_task()?,
        _ => { println!("No command provided"); }
    };

    Ok(())
}
