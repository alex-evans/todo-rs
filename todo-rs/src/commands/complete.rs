
use chrono::prelude::*;
use std::error::Error;
use std::io;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};

use super::list::list_tasks;
use crate::TodoTask;
use crate::Status;
use crate::helpers::helpers::select_task;

pub fn complete_task() -> Result<(), Box<dyn Error>> {
    list_tasks()?;
    println!("\n\nPlease select the Task Id To Complete:\n\n");
    let task_id = select_task()?;

    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt")?;

    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut match_found = false;

    for (index, line) in lines.iter_mut().enumerate() {
        if (index + 1) == task_id {
            let mut task = TodoTask::from_csv(line.to_string())?;
            task.status = Status::Done;
            task.updated_at = Utc::now();
            println!("Task Completed: {}", task.title);
            *line = task.to_csv();
            match_found = true;
            break;
        }
    }

    if !match_found {
        println!("No task found with the given task id");
        return Ok(());
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