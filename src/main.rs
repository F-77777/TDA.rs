use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{from_reader, Value};
use std::any::Any;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io as lmao;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::io::{Seek, SeekFrom};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct BaseTasks {
    in_progress_tasks: Vec<String>,
    completed_tasks: Vec<String>,
    pending_tasks: Vec<String>,
}

fn main() {
    let gen_color = (50, 168, 158);
    let path = Path::new("tasks");
    if !path.exists() {
        println!("{}", "Test 1 failed: tasks folder does not exist!".red());
        println!("{}", "Creating tasks folder...".yellow());
        match fs::create_dir_all("tasks") {
            Ok(_) => {
                println!(
                    "{}",
                    "Tasks folder has successfully been created!".green().bold()
                );
                thread::sleep(Duration::from_millis(300));
            }
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "There has been an error creating the tasks folder:".red(),
                    format!("{e}").bright_red()
                );
                std::process::exit(0);
            }
        }
    } else {
        println!("{}", "Test 1 passed ✅".green());
        thread::sleep(Duration::from_millis(500));
    }
    let mut fil: Option<File> = None;
    while fil.is_none() {
        match OpenOptions::new()
            .read(true)
            .write(true)
            .open("tasks/tasks.json")
        {
            Ok(file) => {
                println!("{}", "Test 2 passed ✅".green());
                thread::sleep(Duration::from_millis(300));
                let fil_ref = &mut fil;
                *fil_ref = Some(file);
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        println!("{}", "Tasks.json does not exist!".red());
                        thread::sleep(Duration::from_millis(200));
                        println!("{}", "Attempting to create tasks.json file".yellow());
                        thread::sleep(Duration::from_millis(100));
                        match File::create("tasks/tasks.json") {
                            Ok(file) => {
                                println!(
                                    "{}",
                                    "Successfully created tasks.json file!".green().bold()
                                );
                                thread::sleep(Duration::from_millis(300));
                                let x = BaseTasks {
                                    in_progress_tasks: Vec::new(),
                                    completed_tasks: Vec::new(),
                                    pending_tasks: Vec::new(),
                                };
                                let _ = serde_json::to_writer(&file, &x);
                                println!("{}", "Test 2 passed ✅".green());
                                let fil_ref = &mut fil;
                                *fil_ref = Some(file);
                            }
                            Err(e) => {
                                eprintln!(
                                    "{}{}",
                                    "There has been an error creating the tasks file:".red(),
                                    format!("{e}").bright_red()
                                );
                            }
                        }
                    }
                    _ => {
                        eprintln!(
                            "{} {}",
                            "There has been an error opening the tasks file:".red(),
                            format!("{e}").bright_red()
                        );
                    }
                };
            }
        }
    }
    print!("\x1B[2J\x1B[H");
    lmao::stdout().flush().unwrap();
    println!(
        "\n{}",
        "This is a todo list program"
            .truecolor(gen_color.0, gen_color.1, gen_color.2)
            .bold()
    );
    loop {
        println!("  1 to add a task");
        println!("  2 to remove a task");
        println!("  3 to view tasks");
        println!("  4 to exit");
        print!("{}", "  Pick a choice: ".blue().bold());
        let mut choice = String::new();
        lmao::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("There was an error reading input!");
        match choice.trim() {
            "1" | "2" | "3" | "4" => {}
            _ => {
                println!("{}", "   Invalid choice!\n".yellow());
                continue;
            }
        }
        let choice: i8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "There was an error!\n".red(),
                    format!("Err: {e}").bright_red()
                );
                continue;
            }
        };
        match choice {
            3 => loop {
                let mut data = String::new();
                if let Some(file) = fil.as_mut() {
                    file.seek(SeekFrom::Start(0)).expect("Failed to seek file");
                    match file.read_to_string(&mut data) {
                        Ok(_) => {
                            if data.trim().is_empty() {
                                eprintln!("{}", "   The file is empty!".yellow());
                                println!("{}", data);
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!(
                                "{}{}",
                                "There was an error reading the file: \n".red(),
                                format!("{e}").bright_red()
                            );
                            continue;
                        }
                    };
                }



                let task_list: BaseTasks = match serde_json::from_str(&data) {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!(
                            "{}{}",
                            "There was an error: \n".red(),
                            format!("{e}").bright_red()
                        );
                        continue;
                    }
                };
                println!("   {}", "1 to display In-progress tasks");
                println!("   {}", "2 to display completed tasks");
                println!("   {}", "3 to display idle tasks");
                println!("   {}", "4 to display all tasks");
                println!("   {}", "5 to exit");
                print!("   {}", "Pick a choice: ".blue().bold());
                let mut choice = String::new();
                lmao::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut choice)
                    .expect("There was an error reading input!");
                let choice: i8 = match choice.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        eprintln!("{}", "You did not enter a valid choice!\n".red());
                        continue;
                    }
                };
                match choice {
                    1 => {
                        if task_list.in_progress_tasks.is_empty() {
                            println!("    No in-progress tasks.");
                        } else {
                            println!(
                                "   {}",
                                "In-progress tasks:".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            for task in &task_list.in_progress_tasks {
                                println!("    {}", task);
                            }
                            continue;
                        }
                        continue;
                    }
                    2 => {
                        if task_list.completed_tasks.is_empty() {
                            println!("     No completed tasks.");
                        } else {
                            println!(
                                "   {}",
                                "Completed tasks:".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for task in &task_list.completed_tasks {
                                println!("    {}", task);
                            }
                            continue;
                        }
                        continue;
                    }
                    3 => {
                        if task_list.pending_tasks.is_empty() {
                            println!("    No Pending tasks.");
                        } else {
                            println!(
                                "   {}",
                                "Pending tasks:".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for task in &task_list.pending_tasks {
                                println!("    {}", task);
                            }
                            continue;
                        }
                        continue;
                    }
                    4 => {
                        println!(
                            "   {}",
                            "\nAll tasks:"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );

                        if task_list.pending_tasks.is_empty() {
                            println!("    No Pending tasks.");
                        } else {
                            println!(
                                "    {}",
                                "Pending Tasks: ".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for task in &task_list.pending_tasks {
                                println!("    {}", task);
                            }
                        }

                        if task_list.in_progress_tasks.is_empty() {
                            println!("    No in-progress tasks.");
                        } else {
                            println!(
                                "    {}",
                                "In-progress Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            for task in &task_list.in_progress_tasks {
                                println!("    {}", task);
                            }
                        }

                        if task_list.completed_tasks.is_empty() {
                            println!("    No Completed tasks.");
                            continue;
                        } else {
                            println!(
                                "    {}",
                                "Completed Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            for task in &task_list.completed_tasks {
                                println!("    {}", task);
                            }
                        }
                        continue;
                    }
                    5 => {
                        println!("{}", "Successfully exited".green());
                        break;
                    }
                    _ => {
                        println!("{}", "Invalid choice!\n".red());
                    }
                }
            },
            4 => {
                println!("\n{}", "Farewell!".green().bold());
                thread::sleep(Duration::from_secs(3));
                std::process::exit(0);
            }
            _ => {
                println!("{}", "   Invalid choice!\n".yellow().bold());
                continue;
            }
        }
    }
}
