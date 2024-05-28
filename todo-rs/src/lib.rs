
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

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    if config.add {
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
    };

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