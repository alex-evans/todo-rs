
use chrono::prelude::*;
use std::error::Error;
use std::io;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};

use super::list::list_tasks;
use crate::TodoTask;

pub fn edit_task() -> Result<(), Box<dyn Error>> {
    list_tasks()?;
    println!("\n\nPlease select the Task Id To Edit:\n\n");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read line");

    let task_id: String = match task_id.trim().parse() {
        Ok(task_id) => task_id,
        Err(_) => {
            println!("Please enter a valid task id");
            return Ok(());
        }
    };

    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt")?;

    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    for (index, line) in lines.iter_mut().enumerate() {
        if (index + 1) == task_id.parse::<usize>().unwrap() {
            
            let mut task = TodoTask::from_csv(line.to_string())?;

            // Title
            println!("Enter new title:");
            let mut title = String::new();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read line");
            task.title = title.trim().to_string();
            
            // Description
            println!("Enter new description:");
            let mut description = String::new();
            io::stdin()
                .read_line(&mut description)
                .expect("Failed to read line");
            task.description = description.trim().to_string();
            
            task.updated_at = Utc::now();
            println!("Task Updated: {}", task.title);
            *line = task.to_csv();
            break;
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")?;

    for line in &lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}