mod misc;
mod taskmod;
use colored::Colorize;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use misc::{
    functions::{input, is_valid_status},
    vals::{
        EditStatus, StatusCode, TaskStatus, FILEPATH as filepath, FOLDER_PATH as folder_path,
        GEN_COLOR as gen_color, MAX_TASKS as max_tasks,
    },
};
use taskmod::taskmod::BaseTasks;
fn main() {
    const _UNUSED_VAR: &str = "║ ╠ ╦ ═ ╚ ╔ ╩";
    let (sender, receiver) = mpsc::channel();
    let shared_sender = Arc::new(Mutex::new(sender));
    let shared_sender_clone = Arc::clone(&shared_sender);
    let mut v = 0;
    let thread1 = thread::spawn(move || {
        let mut last_modified_time: Option<SystemTime> = None;
        loop {
            if v == 0 {
                v += 1;
            }
            let path = Path::new(folder_path);
            // Check if the tasks directory exists, and create it if not.
            if !path.exists() {
                println!("{}", "Test 1 failed: tasks folder does not exist!".red());
                // Create the tasks directory.
                // Create the tasks folder.
                println!("{}", "Creating tasks folder...".yellow());
                match fs::create_dir_all(folder_path) {
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
                } // Create tasks directory.
            } else {
            }
            if Path::new(filepath).exists() {
            } else {
                println!("{}", "tasks.json does not exist!".red());

                if let Err(e) = File::create(filepath).and_then(|file| {
                    BaseTasks::new(&file);
                    Ok(())
                }) {
                    eprintln!(
                        "{} {}",
                        "Error creating or initializing tasks.json:".red(),
                        e.to_string().bright_red()
                    );
                    continue;
                }

                if let Ok(sender) = shared_sender_clone.lock() {
                    sender.send(EditStatus::Authorized).unwrap_or_else(|e| {
                        eprintln!("Error sending message: {}", e);
                    });
                }

                println!("{}", "tasks.json created and initialized.".green());
            }
            if let Ok(mut file) = File::open(filepath) {
                let mut data = String::new();
                if let Err(e) = file.read_to_string(&mut data) {
                    eprintln!(
                        "{} {}",
                        "Error reading tasks.json:".red(),
                        e.to_string().bright_red()
                    );
                    continue;
                }

                if data.trim().is_empty() {
                    println!("{}", "tasks.json is empty!".yellow());

                    if let Err(e) = File::create(filepath).and_then(|file| {
                        BaseTasks::new(&file);
                        Ok(())
                    }) {
                        eprintln!(
                            "{} {}",
                            "Error reinitializing tasks.json:".red(),
                            e.to_string().bright_red()
                        );
                        continue;
                    }

                    if let Ok(sender) = shared_sender_clone.lock() {
                        sender.send(EditStatus::Authorized).unwrap_or_else(|e| {
                            eprintln!("Error sending message: {}", e);
                        });
                    }

                    println!("{}", "tasks.json reinitialized.".green());
                }
            }
            if let Ok(metadata) = fs::metadata(filepath) {
                if let Ok(modified_time) = metadata.modified() {
                    if let Some(last_time) = last_modified_time {
                        if modified_time > last_time
                            && receiver.recv().unwrap() == (EditStatus::Unauthorized)
                        {
                            println!("tasks.json has been externally modified!");

                            if let Ok(sender) = shared_sender_clone.lock() {
                                sender.send(EditStatus::Unauthorized).unwrap_or_else(|e| {
                                    eprintln!("Error sending message: {}", e);
                                });
                            }
                        }
                    }
                    last_modified_time = Some(modified_time);
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });
    thread::sleep(Duration::from_secs(2));
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
    println!(
        "{}",
        r#"
                
        ▄▄▄█████▓▓█████▄  ▄▄▄            ██▀███    ██████ 
        ▓  ██▒ ▓▒▒██▀ ██▌▒████▄         ▓██ ▒ ██▒▒██    ▒ 
        ▒ ▓██░ ▒░░██   █▌▒██  ▀█▄       ▓██ ░▄█ ▒░ ▓██▄   
        ░ ▓██▓ ░ ░▓█▄   ▌░██▄▄▄▄██      ▒██▀▀█▄    ▒   ██▒
          ▒██▒ ░ ░▒████▓  ▓█   ▓██▒ ██▓ ░██▓ ▒██▒▒██████▒▒
          ▒ ░░    ▒▒▓  ▒  ▒▒   ▓▒█░ ▒▓▒ ░ ▒▓ ░▒▓░▒ ▒▓▒ ▒ ░
            ░     ░ ▒  ▒   ▒   ▒▒ ░ ░▒    ░▒ ░ ▒░░ ░▒  ░ ░
          ░       ░ ░  ░   ░   ▒    ░     ░░   ░ ░  ░  ░  
                    ░          ░  ░  ░     ░           ░  
                  ░                  ░                    
    "#
        .truecolor(gen_color.0, gen_color.1, gen_color.2)
    );
    let mut x = 0;
    loop {
        let task_list: Vec<(u16, String, TaskStatus)> =
            BaseTasks::read_tasks(&File::open(filepath).unwrap()).0;
        /* BaseTasks::read_tasks(&file).0;*/
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
        let choice = input(vec!['1', '2', '3', '4', '5', '6']);
        match choice {
            '1' => {
                let mut tries = 0;
                loop {
                    let mut taskname: String = String::new();
                    print!(
                        "{}",
                        "[?] Enter task name > ".truecolor(gen_color.0, gen_color.1, gen_color.2)
                    );
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut taskname)
                        .expect("Failed to read input");
                    let mut task_status = String::new();
                    println!(
                        "{}",
                        "[?] Enter task status (Pending/Completed/In-progress)".truecolor(
                            gen_color.0,
                            gen_color.1,
                            gen_color.2
                        )
                    );
                    println!(
                        "{}",
                        "[?] > ".truecolor(gen_color.0, gen_color.1, gen_color.2)
                    );
                    io::stdout().flush().unwrap();
                    // io::stdin().read_line(&mut task_status).expect("Failed to read input");
                    // let task_status: TaskStatus = match task_status.parse()
                    let mut file = match OpenOptions::new().write(true).append(true).open(filepath)
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
                    file.seek(SeekFrom::Start(1)).unwrap();
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
                        None => continue,
                    }
                }
            }

            '3' => loop {
                println!(
                    "\n{}",
                    "   ║ ".truecolor(gen_color.0, gen_color.1, gen_color.2)
                );
                println!(
                    "{}",
                    " ╔═╬═[1]: Display In-progress tasks".truecolor(
                        gen_color.0,
                        gen_color.1,
                        gen_color.2
                    )
                );
                println!(
                    "{}",
                    " ║ ╠═[2]: Display completed tasks".truecolor(
                        gen_color.0,
                        gen_color.1,
                        gen_color.2
                    )
                );
                println!(
                    "{}",
                    " ║ ╠═[3]: Display pending tasks".truecolor(
                        gen_color.0,
                        gen_color.1,
                        gen_color.2
                    )
                );
                println!(
                    "{}",
                    " ║ ╠═[4]: Display all tasks".truecolor(gen_color.0, gen_color.1, gen_color.2)
                );
                println!(
                    "{}",
                    " ║ ╚═[5]: Exit".truecolor(gen_color.0, gen_color.1, gen_color.2)
                );
                println!("{}", " ║".truecolor(gen_color.0, gen_color.1, gen_color.2));
                print!(
                    "{}",
                    " ╚╦═══[?] > ".truecolor(gen_color.0, gen_color.1, gen_color.2)
                );
                let choice = input(vec!['1', '2', '3', '4', '5']);
                match choice {
                    '1' => {
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
                            let mut tasknum = 0;
                            for (taskid, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::InProgress {
                                    tasknum += 1;
                                    println!(
                                        "        {}",
                                        format!("Task {tasknum}: {task_name} | ID: {taskid}")
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    );
                                }
                            }
                            continue;
                        }
                        continue;
                    }
                    '2' => {
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
                            let mut tasknum: u16 = 0;
                            println!(
                                "   {}",
                                "Completed tasks:".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for (taskid, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::Complete {
                                    tasknum += 1;
                                    println!(
                                        "        {}",
                                        format!("Task {tasknum}: {task_name} | ID: {taskid} ")
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    );
                                }
                            }
                            continue;
                        }
                        continue;
                    }
                    '3' => {
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
                                if *task_status == TaskStatus::Pending {
                                    tasknum += 1;
                                    println!(
                                        "        {}",
                                        format!("Task {tasknum}: {task_name} | ID: {taskid} ")
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    );
                                }
                            }
                            continue;
                        }
                        continue;
                    }
                    '4' => {
                        println!(
                            "{}",
                            "\n  ╠═╦═[Tasks] All tasks:"
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
                            let mut tasknum: u16 = 0;
                            println!(
                                "  {}",
                                "║ ╠══[Pending] Pending Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            println!(
                                "  {}",
                                "║ ╠═╗".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for (taskid, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::Pending {
                                    tasknum += 1;
                                    println!(
                                        "  {}",
                                        format!(
                                            "║ ║ ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                        )
                                        .truecolor(
                                            gen_color.0,
                                            gen_color.1,
                                            gen_color.2
                                        )
                                    );
                                }
                            }
                            println!(
                                "      {}",
                                "╚═══".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                        }

                        if !task_list
                            .iter()
                            .any(|(_, _, status)| *status == TaskStatus::InProgress)
                        {
                            println!(
                                "  {}",
                                "║ ╠══[In-Progress] No In-Progress Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                        } else {
                            let mut tasknum: u16 = 0;
                            println!(
                                "  {}",
                                "║ ╠══[In-Progress] In-Progress Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            println!(
                                "  {}",
                                "║ ╚═╗".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            for (taskid, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::InProgress {
                                    tasknum += 1;
                                    println!(
                                        "  {}",
                                        format!(
                                            "║   ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                        )
                                        .truecolor(
                                            gen_color.0,
                                            gen_color.1,
                                            gen_color.2
                                        )
                                    );
                                }
                            }
                            println!(
                                "      {}",
                                "╚═══".truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
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
                            let mut tasknum: u16 = 0;
                            println!(
                                "    {}",
                                "Completed Tasks: ".truecolor(
                                    gen_color.0,
                                    gen_color.1,
                                    gen_color.2
                                )
                            );
                            for (taskid, task_name, task_status) in &task_list {
                                if *task_status == TaskStatus::Complete {
                                    tasknum += 1;
                                    println!(
                                        "     {}",
                                        format!("Task {tasknum}: {task_name} | ID: {taskid} ")
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    );
                                }
                            }
                        }
                        continue;
                    }
                    '5' => {
                        println!("{}", "Successfully exited".green());
                        break;
                    }
                    _ => {
                        println!("{}", "Invalid choice!\n".red());
                    }
                }
            },
            '5' => {
                println!(
                    "\n{}{}",
                    "   ╚══[:D] "
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold(),
                    "Goodbye!".green().bold()
                );
                thread::sleep(Duration::from_secs(1));
                std::process::exit(0);
            }
            '6' => {
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
                    "     ║        1000 tasks at max | use your brain"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                println!(
                    "{}",
                    "     ║        please don't fucking edit the tasks folder or its contents"
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
    }
}
