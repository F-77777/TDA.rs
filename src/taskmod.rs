pub mod taskmod {
    use crate::misc::vals::{
        StatusCode, TaskStatus, FILEPATH as filepath, GEN_COLOR as gen_color,
        MAX_TASKS as max_tasks,
    };
    use colored::Colorize;
    use serde_json::{self, from_reader, to_writer, Value};
    use std::fs::File;
    use std::io::{BufReader, Write};
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct BaseTasks {
        pub alltasks: Vec<(u16, String, TaskStatus)>,
    }

    impl BaseTasks {
        pub fn new(mut writer: &File) {
            let x = BaseTasks {
                alltasks: Vec::new(),
            };
            let z = serde_json::to_string_pretty(&x).unwrap();
            writer.write_all(z.as_bytes()).unwrap();
        }

        pub fn add_task(
            writer: &File,
            taskstruct: &mut BaseTasks,
            task_name: String,
            status: TaskStatus,
        ) -> Option<StatusCode> {
            if taskstruct.alltasks.len() >= max_tasks {
                println!(
                    "{}\n{}",
                    "You have added too many tasks!".yellow().bold(),
                    "The max amount of tasks is 1000!".yellow().bold()
                );
                return Some(StatusCode::Break);
            }

            let reader = BufReader::new(File::open(filepath).unwrap());
            let mut task_data: BaseTasks = match from_reader(reader) {
                Ok(task_data) => task_data,
                Err(e) => {
                    eprintln!(
                        "{}{}",
                        "Error reading tasks file: ".red(),
                        e.to_string()
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    );
                    return Some(StatusCode::Break);
                }
            };

            task_data
                .alltasks
                .push((task_data.alltasks.len() as u16 + 1, task_name, status));
            taskstruct.alltasks = task_data.alltasks.clone();
            let task_data2 = serde_json::to_string_pretty(&task_data).unwrap();
            // Write the updated tasks back to the file
            match File::create(filepath) {
                Ok(mut file) => match file.write_all(task_data2.as_bytes()) {
                    Ok(_) => {
                        println!(
                            "\n{}{}",
                            "   ╠═".truecolor(gen_color.0, gen_color.1, gen_color.2),
                            "[+] Successfully added task!".green()
                        );
                        println!(
                            "{}",
                            "  ╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2)
                        );
                        Some(StatusCode::Break)
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
            let buf_reader = BufReader::new(reader);
            let mut y = 0;
            let task_list: Result<BaseTasks, _> = from_reader(buf_reader);
            loop {
                y += 1;
                match task_list {
                    Ok(task_data) => {
                        return (task_data.alltasks, None);
                    }
                    Err(ref e) => {
                        eprintln!("{}{}", "Error reading tasks file: ".red(), e);
                        if y == 3 {
                            return (Vec::new(), Some(StatusCode::Break));
                        }
                    }
                };
            }
        }
    }
}
