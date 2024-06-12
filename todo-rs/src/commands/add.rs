
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn test_add_task_valid_input() {
        // Arrange
        let input = "Task Title\n".as_bytes();
        let mut output = Vec::new();
        let expected_task = TodoTask::new("Task Title".to_string(), "Task Description".to_string());

        // Act
        io::stdin().read_to_end(&mut input.to_vec()).unwrap();
        io::stdout().write_all(&mut output).unwrap();
        let result = add_task();

        // Assert
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string("tasks.txt").unwrap(), format!("{}, {}, {:?}, {}, {}\n", expected_task.title, expected_task.description, expected_task.status, expected_task.created_at, expected_task.updated_at));
    }

    #[test]
    fn test_add_task_invalid_title() {
        // Arrange
        let input = "\n".as_bytes();
        let mut output = Vec::new();

        // Act
        io::stdin().read_to_end(&mut input.to_vec()).unwrap();
        io::stdout().write_all(&mut output).unwrap();
        let result = add_task();

        // Assert
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string("tasks.txt").unwrap(), "");
    }

    #[test]
    fn test_add_task_invalid_description() {
        // Arrange
        let input = "Task Title\n\n".as_bytes();
        let mut output = Vec::new();

        // Act
        io::stdin().read_to_end(&mut input.to_vec()).unwrap();
        io::stdout().write_all(&mut output).unwrap();
        let result = add_task();

        // Assert
        assert!(result.is_ok());
        assert_eq!(fs::read_to_string("tasks.txt").unwrap(), "");
    }
}