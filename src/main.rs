use colored::{ColoredString, Colorize};
use serde_json;
use serde_json::{from_reader, to_writer, Value};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self as lmao, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
mod taskmod;
use taskmod::TaskMod::{BaseTasks, StatusCode, TaskStatus};
fn main() {
    let thread1 = thread::spawn( || {
        let mut fil: Option<File> = None;
        loop {
            thread::sleep(Duration::from_millis(20));
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
                    }
                    Err(e) => {
                        eprintln!(
                            "{} {}",
                            "There has been an error creating the tasks folder:".red(),
                            format!("{e}").bright_red()
                        );
                        continue;
                    }
                }
            } else {
            }
            if Path::new("tasks/tasks.json").exists() {
                println!("The file '{}' exists.", file_path);
            } else {
                println!("The file '{}' does not exist.", file_path);
            }
            while fil.is_none() {
                match OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open("tasks/tasks.json")
                {
                    Ok(file) => {
                        fil = Some(file);
                    }
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::NotFound => {
                            println!("{}", "Tasks.json does not exist!".red());
                            println!("{}", "Attempting to create tasks.json file".yellow());
                            match File::create("tasks/tasks.json") {
                                Ok(file) => {
                                    println!(
                                        "{}",
                                        "Successfully created tasks.json file!".green().bold()
                                    );
                                    fil = Some(file);
                                    BaseTasks::new(fil.as_ref().unwrap());
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
                    },
                }
            }
            let mut data = String::new();
            loop {
                if let Some(file) = fil.as_mut() {
                    file.seek(SeekFrom::Start(0)).expect("Failed to seek file");
                    match &file.read_to_string(&mut data) {
                        Ok(_) => {
                            if data.trim().is_empty() {
                                if let Some(file_ref) = fil.as_ref() {
                                    BaseTasks::new(file_ref);
                                }
                            }
                            break;
                        }
                        Err(_) => {
                            break;
                        }
                    };
                }
            }
        }
    });
    thread::sleep(Duration::from_secs(5));
    let gen_color = (245, 66, 105);

    print!("\x1B[2J\x1B[H");
    lmao::stdout().flush().unwrap();
    println!(
        "{}",
        r#"
                __      .___                         
              _/  |_  __| _/____      _______  ______
              \   __\/ __ |\__  \     \_  __ \/  ___/
               |  | / /_/ | / __ \_    |  | \/\___ \ 
               |__| \____ |(____  / /\ |__|  /____  >
                        \/     \/  \/            \/ 
    "#
        .truecolor(gen_color.0, gen_color.1, gen_color.2)
    );
    let mut x = 0;
    loop {
        let file = File::open("tasks/tasks.json").unwrap();
        let task_list: Vec<(u16, String, TaskStatus)> = BaseTasks::read_tasks(&file).0;
        if x == 0 {
            println!("");
            println!(
                "{}",
                "╔═╦[1]: Add Task".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            println!(
                "{}",
                "║ ╠[2]: Remove Task".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            println!(
                "{}",
                "║ ╠[3]: Display Tasks".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            println!(
                "{}",
                "║ ╠[4]: Edit Task".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            println!(
                "{}",
                "║ ╠[5]: Exit".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            println!(
                "{}",
                "║ ╚[6]: About".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            println!("{}", "║".truecolor(gen_color.0, gen_color.1, gen_color.2));
            print!(
                "{}",
                "╚══╦═[?] > ".truecolor(gen_color.0, gen_color.1, gen_color.2)
            );
            x += 1;
        } else {
            println!(
                "{}",
                "╔═╩╦[1]: Add Task"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╠[2]: Remove Task"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╠[3]: Display Tasks"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╠[4]: Edit Task"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╠[5]: Exit"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╚[6]: About"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
            );
            print!(
                "{}",
                "╚══╦═[?] > "
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
        }
        let mut choice = String::new();
        lmao::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("There was an error reading input!");
        match choice.trim() {
            "1" | "2" | "3" | "4" | "5" | "6" => {}
            _ => {
                println!(
                    "{}",
                    "   ║"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}{}",
                    "   ╠══[!]:"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold(),
                    " Invalid choice!".yellow().bold()
                );
                println!(
                    "{}",
                    "  ╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2)
                );
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
            1 => loop {
                let mut tries = 0;
                let file = match OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open("tasks/tasks.json")
                {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!(
                            "{} {}",
                            "There was an error!\n".red(),
                            format!("Err: {e}").bright_red()
                        );
                        tries += 1;
                        if tries > 3 {
                            break;
                        }
                        continue;
                    }
                };
                let res = BaseTasks::add_task(
                    &file,
                    &mut BaseTasks {
                        alltasks: task_list.clone(),
                    },
                    String::new(),
                    TaskStatus::Pending,
                );
                match res {
                    Some(StatusCode::Break) => break,
                    Some(StatusCode::Continue) => continue,
                    Some(StatusCode::EndProcess) => std::process::exit(0),
                    None => {
                        std::process::exit(0);
                    }
                }
            },

            3 => loop {
                println!("   {}", "1 to display In-progress tasks");
                println!("   {}", "2 to display completed tasks");
                println!("   {}", "3 to display pending tasks");
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
                    Err(_) => {
                        eprintln!("{}", "You did not enter a valid choice!\n".red());
                        continue;
                    }
                };
                match choice {
                    1 => {
                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::InProgress)
                        {
                            println!("    No in-progress tasks.");
                        } else {
                            println!("   {}", "".truecolor(gen_color.0, gen_color.1, gen_color.2));
                            println!(
                                "   {}",
                                "╠[TASKS] In-progress tasks:".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            let mut z = 0;
                            for (tasknum, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::InProgress {
                                    z += 1;
                                    println!("    Task {z}: {} | ID: {tasknum}", task_name);
                                }
                            }
                            continue;
                        }
                        continue;
                    }
                    2 => {
                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::Complete)
                        {
                            println!(
                                "{}",
                                "     No completed tasks".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                        } else {
                            println!(
                                "   {}",
                                "Completed tasks:".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for (tasknum, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::Complete {
                                    println!("     {}", task_name);
                                }
                            }
                            continue;
                        }
                        continue;
                    }
                    3 => {
                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::Pending)
                        {
                            println!(
                                "{}",
                                "     No Pending tasks".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                        } else {
                            println!(
                                "   {}",
                                "Pending tasks:".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            let mut tasknum: u16 = 0;
                            for (taskid, task_name, task_status) in &task_list {
                                tasknum += 1;
                                if *task_status == TaskStatus::Pending {
                                    println!(
                                        "     {}{} {}",
                                        format!("Task {tasknum}"),
                                        task_name,
                                        format!("ID: {}", taskid).magenta()
                                    );
                                }
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

                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::Pending)
                        {
                            println!(
                                "{}",
                                "     No Pending tasks".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                        } else {
                            println!(
                                "    {}",
                                "Pending Tasks: ".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );

                            for (tasknum, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::Pending {
                                    println!("     {}", task_name);
                                }
                            }
                        }

                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::InProgress)
                        {
                            println!(
                                "{}",
                                "     No in-progress tasks".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                        } else {
                            println!(
                                "    {}",
                                "In-progress Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            for (tasknum, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::InProgress {
                                    println!("     {}", task_name);
                                }
                            }
                        }

                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::Complete)
                        {
                            println!(
                                "{}",
                                "     No completed tasks".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
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
                            for (tasknum, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::Complete {
                                    println!(
                                        "     {}{}",
                                        format!("Task {tasknum}: ").truecolor(
                                            gen_color.0,
                                            gen_color.1,
                                            gen_color.2
                                        ),
                                        task_name
                                    );
                                }
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
            5 => {
                println!(
                    "{}",
                    "   ║"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}{}",
                    "   ╚══[:D] "
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold(),
                    "Goodbye!".green().bold()
                );
                thread::sleep(Duration::from_secs(1));
                std::process::exit(0);
            }
            6 => {
                println!(
                    "{}",
                    "   ║"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}",
                    "   ╚═╦═[Info]: TDA.rs(Todoapp.rs) is a todo list"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}",
                    "     ║        app made by F - 77777 (0xlryx#0 on discord)"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}",
                    "     ║        made with rust (With a few aesthetics)"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}",
                    "   ╔═╝        github.com/F-77777/TDA.rs"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}",
                    "  ╔╝"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
            }
            _ => {
                println!("{}", "   Invalid choice!\n".yellow().bold());
                continue;
            }
        }
    }
}
