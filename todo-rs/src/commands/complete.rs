
use chrono::prelude::*;
use std::error::Error;
use std::io;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};

use super::list::list_tasks;
use crate::TodoTask;
use crate::Status;

pub fn complete_task() -> Result<(), Box<dyn Error>> {
    list_tasks()?;
    println!("\n\nPlease select the Task Id To Complete:\n\n");
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
            task.status = Status::Done;
            task.updated_at = Utc::now();
            println!("Task Completed: {}", task.title);
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

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}