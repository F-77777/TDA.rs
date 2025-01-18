pub mod TaskMod {
    use colored::{ColoredString, Colorize};
    use serde_json::{from_reader, to_writer, Value};
    use std::fs::File;
    use std::io::{self as lmao, BufWriter, Read, Seek, SeekFrom, Write};
    #[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub enum TaskStatus {
        InProgress,
        Complete,
        Pending,
    }
    pub enum StatusCode {
        Continue,
        Break,
        EndProcess,
    }
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct BaseTasks {
        pub alltasks: Vec<(u16, String, TaskStatus)>,
    }
    impl BaseTasks {
        pub fn new(writer: &File) {
            let x = BaseTasks {
                alltasks: Vec::new(),
            };
            match serde_json::to_writer(writer, &x) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "{}{}",
                        "There was an error: ".red(),
                        format!("{e}").bright_red().bold()
                    );
                }
            }
        }
        pub fn add_task(
            writer: &File,
            taskstruct: &mut BaseTasks,
            task_name: String,
            status: TaskStatus,
        ) -> Option<StatusCode> {
            if taskstruct.alltasks.len() > 1000 {
                println!(
                    "{}\n{}",
                    "You have added too many tasks!".yellow().bold(),
                    "The max amount of tasks is 1000!".yellow().bold()
                );
                return Some(StatusCode::Break);
            } else {
            }
            let mut task_data: BaseTasks = match from_reader(writer) {
                Ok(task_data) => task_data,
                Err(e) => {
                    eprintln!("{}{}", "Error reading tasks file: ".red(), e);
                    return Some(StatusCode::Break);
                }
            };
            task_data
                .alltasks
                .push((task_data.alltasks.len() as u16 + 1, task_name, status));
            taskstruct.alltasks = task_data.alltasks.clone();
            // Write the updated tasks back to the file
            let file = File::create("tasks/tasks.json");
            match file {
                Ok(file) => match to_writer(file, &task_data) {
                    Ok(_) => {
                        println!("{}", "Successfully added task!".green());
                        return Some(StatusCode::Break);
                    }
                    Err(e) => {
                        eprintln!("{}{}", "Error writing tasks to file: ".red(), e);
                        Some(StatusCode::Break)
                    }
                },
                Err(e) => {
                    eprintln!("{}{}", "Error creating file: ".red(), e);
                    Some(StatusCode::Break)
                }
            }
        }
        pub fn read_tasks(reader: &File) -> (Vec<(u16, String, TaskStatus)>, Option<StatusCode>) {
            let vec = Vec::new();
            (vec, Some(StatusCode::Break))
        }
    }
}
