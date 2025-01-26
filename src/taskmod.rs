pub mod task_util_mod {
    use crate::misc::{
        functions::input,
        vals::{TaskStatus, FILEPATH as filepath, GEN_COLOR as gen_color},
    };
    use colored::Colorize;
    use serde_json::{self, from_reader};
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, Write};
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct BaseTasks {
        pub alltasks: Vec<(u16, String, TaskStatus)>,
    }

    impl BaseTasks {
        pub fn init(mut writer: &File) {
            let x = BaseTasks {
                alltasks: Vec::new(),
            };
            let z = serde_json::to_string_pretty(&x).unwrap();
            writer.write_all(z.as_bytes()).unwrap();
        }
        pub fn add_task(task_name: String, status: TaskStatus) {
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
                    std::process::exit(0);
                }
            };
            let mut existing_ids: Vec<u16> =
                task_data.alltasks.iter().map(|(id, _, _)| *id).collect();
            existing_ids.sort_unstable();
            let mut taskid: u16 = 1;
            for &id in &existing_ids {
                if taskid < id {
                    break;
                }
                taskid = id + 1;
            }

            task_data
                .alltasks
                .push((taskid, task_name.trim().to_string().clone(), status.clone()));
            let task_data2 = serde_json::to_string_pretty(&task_data).unwrap();
            match File::create(filepath) {
                Ok(mut file) => match file.write_all(task_data2.as_bytes()) {
                    Ok(_) => {
                        println!(
                            "{}",
                            "    ╚═╗"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold(),
                        );
                        println!(
                            "{}{}",
                            "      ╠══[+]".truecolor(gen_color.0, gen_color.1, gen_color.2),
                            " Successfully added task ".bright_green().bold(),
                        );
                        println!(
                            "{}{}",
                            "      ╠═[Name]:".truecolor(gen_color.0, gen_color.1, gen_color.2),
                            format!(" {}", task_name.trim()).bright_green().bold(),
                        );
                        println!(
                            "{}{}",
                            "      ╠═[Status]:".truecolor(gen_color.0, gen_color.1, gen_color.2),
                            format!(" {status:?} | ID: {taskid}").bright_green().bold(),
                        );
                    }
                    Err(e) => {
                        eprintln!(
                            "{}{}",
                            "Error writing tasks to file: ".red().bold(),
                            e.to_string()
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                    }
                },
                Err(e) => {
                    eprintln!("{}{}", "Error creating file: ".red(), e);
                }
            }
        }

        pub fn read_tasks(reader: &File) -> Vec<(u16, String, TaskStatus)> {
            let buf_reader = BufReader::new(reader);
            let mut y = 0;
            let task_list: Result<BaseTasks, _> = from_reader(buf_reader);
            loop {
                y += 1;
                match task_list {
                    Ok(task_data) => {
                        return task_data.alltasks;
                    }
                    Err(ref e) => {
                        eprintln!("{}{:?}", "Error reading tasks file from ln 91: ".red(), e);

                        if y == 3 {
                            let f1 = File::open(filepath).unwrap();
                            BaseTasks::init(&f1);
                            break;
                        }
                    }
                }
            }
            Vec::new()
        }
        pub fn remove_task(task_id: u16) -> char {
            let mut task_data = BaseTasks::read_tasks(&File::open(filepath).unwrap());
            if let Some(pos) = task_data.iter().position(|&(i, _, _)| i == task_id) {
                task_data.remove(pos);
                let task_data2 = serde_json::to_string_pretty(&BaseTasks {
                    alltasks: task_data,
                })
                .unwrap();
                let mut f2 = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(filepath)
                    .unwrap();
                match f2.write_all(task_data2.as_bytes()) {
                    Ok(_) => {
                        print!(
                            "    {}{}",
                            "╠═".truecolor(gen_color.0, gen_color.1, gen_color.2).bold(),
                            "[-] Successfully removed task".bright_green().bold()
                        );
                        't'
                    }
                    Err(e) => {
                        eprintln!("{}{}", "Error writing tasks to file: ".red(), e);
                        'f'
                    }
                }
            } else {
                println!(
                    "    {}",
                    format!("╠═[!] Task With the ID {task_id} does not exist")
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                print!(
                    "\n     {}",
                    "╠═[?] Are you lost / need instructions? Y/n > "
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                let choice = input(vec!['Y', 'y', 'N', 'n']);
                match choice {
                    'Y' | 'y' => {
                        println!(
                            "\n      {}",
                            "║ You have to enter the unique id of the task you want to remove."
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        println!(
                            "      {}",
                            "║ When you press 3 in the homepage to display tasks"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        println!(
                            "      {}",
                            "║ The status and unique ID of every task is shown"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        println!(
                            "      {}",
                            "║ Also, please don't edit the tasks folder or any of its contents"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                    }
                    'N' | 'n' => {}
                    _ => {}
                }
                'f'
            }
        }
        pub fn edit_task(task_id: u16, new_name: String, new_status: TaskStatus, r1: &File) {
            let mut task_data = BaseTasks::read_tasks(r1);
            let x = Self::check_id(task_id, &task_data);
            match x {
                true => {}
                false => {
                    println!(
                        "    {}",
                        format!("╠══[!] Task With the ID {task_id} does not exist")
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold(),
                    );
                }
            }
            if new_status == TaskStatus::Nth {
                for (id, name, _) in &mut task_data {
                    if *id == task_id {
                        *name = new_name.clone();
                    }
                }
            } else {
                for (id, name, status) in &mut task_data {
                    if *id == task_id {
                        *name = new_name.clone();
                        *status = new_status.clone();
                    }
                }
            }
            let mut f1 = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(filepath)
                .unwrap();
            let task_data2 = serde_json::to_string_pretty(&BaseTasks {
                alltasks: task_data,
            })
            .unwrap();
            match f1.write_all(task_data2.as_bytes()) {
                Ok(_) => {
                    println!(
                        "\n    {}{}",
                        "╠══[-] "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold(),
                        "Successfully edited task".bright_green().bold()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "\n    {}{}",
                        "╠══[!] Error writing tasks to file: "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold(),
                        e.to_string()
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                }
            }
        }
        pub fn check_id(id: u16, contents: &[(u16, String, TaskStatus)]) -> bool {
            contents.iter().any(|(i, _, _)| *i == id)
        }
    }
}
