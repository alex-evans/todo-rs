
use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use crate::TodoTask;

pub fn add_task() -> Result<(), Box<dyn Error>> {
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
