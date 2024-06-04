
use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use chrono::prelude::*;

use crate::Status;

pub fn list_tasks() -> Result<(), Box<dyn Error>> {
    println!("Listing Tasks");

    let file = OpenOptions::new()
        .read(true)
        .open("tasks.txt")?;

    let reader = io::BufReader::new(file);

    let mut counter = 1;

    println!("{:<5} | {:<10} | {:<50} | {:<10} | {:<20} | {:<20}", "ID", "Title", "Description", "Status", "Created At", "Updated At");
    println!("{:<5} | {:<10} | {:<50} | {:<10} | {:<20} | {:<20}", "-----", "----------", "--------------------------------------------------", "----------", "--------------------", "--------------------");
    
    for line in reader.lines() {
        let line = line?;
        let split_line = line.split(", ");
        let parts: Vec<&str> = split_line.collect();
        if parts.len() == 5 {
            let title = parts[0];
            let description = parts[1];
            let status = match parts[2].parse::<Status>() {
                Ok(Status::ToDo) => "Pending",
                Ok(Status::Done) => "Completed",
                _ => {
                    println!("Invalid status");
                    continue;
                }
            };
            let created_at = match parts[3].parse::<DateTime<Utc>>() {
                Ok(dt) => dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                Err(_) => {
                    println!("Invalid created_at");
                    continue;
                }
            };
            let updated_at = match parts[4].parse::<DateTime<Utc>>() {
                Ok(dt) => dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string(),
                Err(_) => {
                    println!("Invalid updated_at");
                    continue;
                }
            };
            println!("{:<5} | {:<10} | {:<50} | {:<10} | {:<20} | {:<20}", counter, title, description, status, created_at, updated_at);
        } else {
            println!("Invalid task format");
        }
        counter += 1;
        
    }

    Ok(())
}