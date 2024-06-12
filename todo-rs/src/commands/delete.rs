
use std::error::Error;
use std::io;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};

use super::list::list_tasks;
use crate::helpers::helpers::select_task;
use crate::TodoTask;

pub fn delete_task() -> Result<(), Box<dyn Error>> {
    list_tasks()?;
    println!("\n\nPlease select the Task Id To Delete:\n\n");
    let task_id = select_task()?;

    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt")?;

    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut new_lines: Vec<String> = Vec::new(); 
    let mut match_found = false;

    for (index, line) in lines.iter_mut().enumerate() {
        if (index + 1) == task_id {
            let task = TodoTask::from_csv(line.to_string())?;
            println!("Task Deleted: {}", task.title);
            match_found = true;
            continue;
        }

        new_lines.push(line.to_string());
    }

    if !match_found {
        println!("No task found with the given task id");
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("tasks.txt")?;
        
    for new_line in &new_lines {
        writeln!(file, "{}", new_line)?;
    }

    Ok(())
}