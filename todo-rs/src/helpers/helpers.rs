
use std::error::Error;
use std::io;

pub fn select_task() -> Result<usize, Box<dyn Error>> {
    println!("\n\nPlease select the Task Id:\n\n");
    let mut task_id = String::new();
    io::stdin()
        .read_line(&mut task_id)
        .expect("Failed to read line");

    let task_id: usize = match task_id.trim().parse() {
        Ok(task_id) => task_id,
        Err(_) => {
            println!("Please enter a valid task id");
            return Ok(0);
        }
    };

    Ok(task_id)
}

