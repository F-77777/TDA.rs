use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{from_reader, Value};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io as lmao;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(serde::Serialize)]
struct BaseTasks {
    inprogresstasks: Vec<String>,
    completedtasks: Vec<String>,
    tasks: Vec<String>,
}

fn main() {
    let gen_color = (50, 168, 158);
    println!(
        "\n{}",
        "This is a todo list program"
            .truecolor(gen_color.0, gen_color.1, gen_color.2)
            .bold()
    );
    let path = Path::new("tasks");
    if !path.exists() {
        println!("{}", "Test 1 failed: tasks folder does not exist!".red());
        match fs::create_dir_all("tasks") {
            Ok(_) => {}
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
                fil = Some(file);
            }
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::NotFound => match File::create("tasks/tasks.json") {
                        Ok(file) => {
                            let x = BaseTasks {
                                inprogresstasks: Vec::new(),
                                completedtasks: Vec::new(),
                                tasks: Vec::new(),
                            };
                            let _ = serde_json::to_writer(&file, &x);
                            println!("{}", "Test 2 passed ✅".green());
                            fil = Some(file);
                        }
                        Err(e) => {
                            eprintln!(
                                "{}{}",
                                "There has been an error creating the tasks file:".red(),
                                format!("{e}").bright_red()
                            );
                        }
                    },
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
    let mut a = BaseTasks {
        inprogresstasks: Vec::new(),
        completedtasks: Vec::new(),
        tasks: Vec::new(),
    };
    println!("{:?}", fil);
    a.tasks.push("sdfgfdsdfhhdfsdf".to_string());
    match serde_json::to_writer(&fil.unwrap(), &a) {
        Ok(_) => {
            println!("{}", "Test 3 passed ✅".green());
        }
        Err(e) => {
            println!("{}", "Test 3 failed ❌".red());
            eprintln!(
                "{} {}",
                "There has been an error writing to the tasks file:".red(),
                format!("{e}").bright_red()
            );
        }
    }
    loop {
        println!("  1 to add a task");
        println!("  2 to remove a task");
        println!("  3 to view all tasks");
        println!("  4 to exit");
        print!("{}", "Pick a choice: ".blue().bold());
        let mut choice = String::new();
        lmao::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("There was an error reading input!");
        match choice.trim() {
            "1" | "2" | "3" | "4" => {}
            _ => {
                println!("{}", "You did not enter a valid choice!\n".red());
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
            3 => {
                let path = "/home/user/randomproj/todoapp/tasks/tasks.json";
                if Path::new("tasks").exists() {
                } else {
                    match fs::create_dir_all("tasks") {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "{} {}",
                                "There has been an error:".red(),
                                format!("{e}").bright_red()
                            );
                            continue;
                        }
                    }
                }
            }
            4 => {
                println!("\n{}", "Farewell!".green().bold());
                thread::sleep(Duration::from_secs(3));
                std::process::exit(0);
            }
            _ => {
                println!("{}", "Invalid choice!".red().bold());
                continue;
            }
        }
    }
}
