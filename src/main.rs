mod misc;
mod taskmod;
use colored::Colorize;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

use misc::{
    functions::{display_warning, input, send_edit_status},
    vals::{
        EditStatus, TaskStatus, FILEPATH as filepath, FOLDER_PATH as folder_path,
        GEN_COLOR as gen_color, MAX_TASKS as max_tasks,
    },
};
use taskmod::task_util_mod::BaseTasks;
fn main() {
    let (sender, receiver) = mpsc::channel();
    let shared_sender = Arc::new(Mutex::new(sender));
    let shared_sender_clone = Arc::clone(&shared_sender);
    let status_acknowledged = Arc::new(AtomicBool::new(true));
    let status_acknowledged_clone = Arc::clone(&status_acknowledged);

    let _thread1 = thread::spawn(move || {
        let mut last_modified_time: Option<SystemTime> = None;

        loop {
            let path = Path::new(folder_path);
            if !path.exists() {
                if let Err(e) = fs::create_dir_all(folder_path) {
                    eprintln!(
                        "{} {}",
                        "There has been an error creating the tasks folder:".red(),
                        format!("{e}").bright_red()
                    );
                    continue;
                }
            }

            if !Path::new(filepath).exists() {
                if let Err(e) = File::create(filepath).map(|file| {
                    BaseTasks::init(&file);
                }) {
                    eprintln!(
                        "{} {}",
                        "Error creating or initializing tasks.json:".red().bold(),
                        e.to_string().bright_red().bold()
                    );
                    continue;
                }

                if let Ok(sender) = shared_sender_clone.lock() {
                    sender.send(EditStatus::Authorized).unwrap_or_else(|e| {
                        eprintln!("Error sending message: {}", e);
                    });
                }
            }

            if let Ok(mut file) = File::open(filepath) {
                let mut data = String::new();
                if file.read_to_string(&mut data).is_err() {
                    continue;
                }

                if data.trim().is_empty() {
                    thread::sleep(Duration::from_millis(300));
                    if data.trim().is_empty() {
                        if let Err(e) = File::create(filepath).map(|file| {
                            BaseTasks::init(&file);
                            if let Ok(sender) = shared_sender_clone.lock() {
                                sender.send(EditStatus::Authorized).unwrap_or_else(|e| {
                                    eprintln!("Error sending message: {}", e);
                                });
                            }
                        }) {
                            eprintln!(
                                "{} {}",
                                "Error reinitializing tasks.json:".red().bold(),
                                e.to_string().bright_red().bold()
                            );
                            continue;
                        }
                    }
                }
            }

            if let Ok(metadata) = fs::metadata(filepath) {
                if let Ok(modified_time) = metadata.modified() {
                    if let Some(last_time) = last_modified_time {
                        if modified_time > last_time {
                            let rec = match receiver.try_recv() {
                                Ok(e) => e,
                                Err(_) => EditStatus::Unauthorized,
                            };

                            match rec {
                                EditStatus::Authorized => {
                                    status_acknowledged.store(true, Ordering::SeqCst);
                                }
                                _ => {
                                    display_warning();
                                }
                            }
                        }
                    }
                    last_modified_time = Some(modified_time);
                }
            }
            thread::sleep(Duration::from_millis(700));
        }
    });
    thread::sleep(Duration::from_secs(3));
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
        .bold()
    );
    let mut x = 0;
    loop {
        let task_list: Vec<(u16, String, TaskStatus)> =
            BaseTasks::read_tasks(&File::open(filepath).unwrap());
        if x == 0 {
            println!("\n");
            println!(
                "{}",
                "╔═╦[1]: Add Task"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║ ╠[2]: Remove Task"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║ ╠[3]: Display Tasks"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║ ╠[4]: Edit Task"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║ ╠[5]: About"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║ ╠[6]: Start over / Clear tasks (if you fucked up the json file or want a fresh start)"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            thread::sleep(Duration::from_millis(500));
            println!(
                "{}",
                "║ ╚═[7]: Exit"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
            );
            thread::sleep(Duration::from_millis(400));
            print!(
                "{}",
                "╚══╦═[?] > "
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
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
                "║  ╠[5]: About"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╠[6]: Start over / Clear tasks "
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║  ╚═[7]: Exit"
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
            println!(
                "{}",
                "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
            );
            thread::sleep(Duration::from_millis(400));
            print!(
                "{}",
                "╚══╦═[?] > "
                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                    .bold()
            );
        }
        let choice = input(vec!['1', '2', '3', '4', '5', '6', '7']);
        match choice {
            '1' => {
                println!(
                    "   {}",
                    "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                );
                loop {
                    let mut taskname: String = String::new();
                    print!(
                        "    {}",
                        "╠══[?] Enter task name > "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut taskname)
                        .expect("Failed to read input");
                    if taskname.trim() == "" {
                        println!(
                            "    {}",
                            "╠══[!] Task name cannot be empty."
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        continue;
                    }

                    loop {
                        println!(
                            "    {}",
                            "║ [1] Pending"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        println!(
                            "    {}",
                            "║ [2] Complete"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        println!(
                            "    {}",
                            "║ [3] In-Progress"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        print!(
                            "    {}",
                            "╠══[?] Task Status > "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        let choice = input(vec!['1', '2', '3']);
                        let status = match choice {
                            '1' => TaskStatus::Pending,
                            '2' => TaskStatus::Complete,
                            '3' => TaskStatus::InProgress,
                            _ => {
                                continue;
                            }
                        };
                        let f2 = File::open(filepath).unwrap();
                        drop(f2);
                        BaseTasks::add_task(taskname.clone(), status);
                        send_edit_status(
                            Arc::clone(&shared_sender),
                            Arc::clone(&status_acknowledged_clone),
                        );
                        break;
                    }
                    print!(
                        "      {}",
                        "╠══[?] Would you like to exit? Y/n > "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    let choice = input(vec!['Y', 'y', 'N', 'n']);
                    match choice {
                        'Y' | 'y' => {
                            println!(
                                "  {}",
                                "╔═══╝"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );
                            break;
                        }
                        'N' | 'n' => {
                            println!(
                                "    {}",
                                "╔═╝"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );
                        }
                        _ => {
                            println!(
                                "      {}",
                                "╠══[_] I wonder how you got this message"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );
                        }
                    }
                }
            }

            '2' => loop {
                println!(
                    "   {}",
                    "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                );
                print!(
                    "    {}",
                    "╠═[?] ID of task to remove > "
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                let mut id_of_task = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_of_task).unwrap();
                let id_of_task: u16 = match id_of_task.trim().parse() {
                    Ok(id) => {
                        if id > max_tasks as u16 {
                            println!(
                                "{}",
                                "ID is out of range: max number of tasks is 1000!"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );
                            continue;
                        }
                        id
                    }
                    Err(_) => {
                        println!(
                            "    {}",
                            "╠═[!] Invalid ID. Please enter a positive integer thats under 1001 "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        println!(
                            "    {}",
                            "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                        );
                        print!(
                            "     {}",
                            "╠═[?] Would you like to exit? Y/n > "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        let choice = input(vec!['Y', 'y', 'N', 'n']);
                        match choice {
                            'Y' | 'y' => {
                                break;
                            }
                            'N' | 'n' => {}
                            _ => {}
                        }
                        print!(
                            "     {}",
                            "╠═[?] Are you lost / need instructions? Y/n > "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        let choice = input(vec!['Y', 'y', 'N', 'n']);
                        match choice {
                            'Y' | 'y' => {
                                println!(
                                    "     {}",
                                    "║ Enter the unique ID of the task you want to remove."
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "     {}",
                                    "║ When you press 3 in the homepage to display tasks"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "     {}",
                                    "║ The unique ID of every task and status is shown"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "     {}",
                                    "║ Also, please don't edit the tasks folder or any of its contents".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                                print!(
                                    "    {}",
                                    "╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                                continue;
                            }
                            'N' | 'n' => {
                                print!(
                                    "    {}",
                                    "╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                                continue;
                            }
                            _ => {}
                        }
                        continue;
                    }
                };
                let action = BaseTasks::remove_task(id_of_task);
                send_edit_status(
                    Arc::clone(&shared_sender),
                    Arc::clone(&status_acknowledged_clone),
                );
                match action {
                    't' => {
                        print!(
                            "    {}",
                            "╠═[?] Would you like to exit? Y/n > "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        let choice = input(vec!['Y', 'y', 'N', 'n']);
                        match choice {
                            'Y' | 'y' => {
                                println!(
                                    "  {}",
                                    "╔═╝"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                break;
                            }
                            'N' | 'n' => {
                                continue;
                            }
                            _ => {}
                        }
                    }
                    'f' => {
                        continue;
                    }
                    _ => {}
                }
            },

            '3' => {
                println!(
                    "   {}",
                    "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                );
                loop {
                    println!(
                        "{}",
                        " ╔═╬[1]: Display In-progress tasks"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "{}",
                        " ║ ╠[2]: Display completed tasks"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "{}",
                        " ║ ╠[3]: Display pending tasks"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "{}",
                        " ║ ╠[4]: Display all tasks"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "{}",
                        " ║ ╚═[5]: Exit"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "{}",
                        " ║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                    );
                    print!(
                        "{}",
                        " ╚╦═╦═[?] > "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    let choice = input(vec!['1', '2', '3', '4', '5']);
                    match choice {
                        '1' => {
                            if !task_list
                                .iter()
                                .any(|(_, _, status)| *status == TaskStatus::InProgress)
                            {
                                println!(
                                    "  {}",
                                    "║ ╠══[_] No In-Progress tasks"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "╚╦╝"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                            } else {
                                let mut tasknum: u16 = 0;
                                println!(
                                    "  {}",
                                    "║ ╠══[In-Progress] In-Progress Tasks: "
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
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
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                            .bold()
                                        );
                                    }
                                }
                                println!(
                                    "  {}",
                                    "║   ╚══[In-Progress]"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                            }
                            continue;
                        }
                        '2' => {
                            if !task_list
                                .iter()
                                .any(|(_, _, status)| *status == TaskStatus::Complete)
                            {
                                println!(
                                    "  {}",
                                    "║ ╠══[_] No Completed tasks".truecolor(
                                        gen_color.0,
                                        gen_color.1,
                                        gen_color.2
                                    )
                                );
                                println!(
                                    "  {}",
                                    "╚╦╝".truecolor(gen_color.0, gen_color.1, gen_color.2)
                                );
                            } else {
                                let mut tasknum: u16 = 0;
                                println!(
                                    "  {}",
                                    "║ ╠══[Completed] Completed Tasks: ".truecolor(
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
                                    if *task_status == TaskStatus::Complete {
                                        tasknum += 1;
                                        println!(
                                            "  {}",
                                            format!(
                                                "║   ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                            )
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        );
                                    }
                                }
                                println!(
                                    "  {}",
                                    "║   ╚══[Completed]".truecolor(
                                        gen_color.0,
                                        gen_color.1,
                                        gen_color.2
                                    )
                                );
                                println!(
                                    "  {}",
                                    "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2)
                                );
                            }
                            continue;
                        }
                        '3' => {
                            if !task_list
                                .iter()
                                .any(|(_, _, status)| *status == TaskStatus::Pending)
                            {
                                println!(
                                    "  {}",
                                    "║ ╠══[_] No Pending tasks".truecolor(
                                        gen_color.0,
                                        gen_color.1,
                                        gen_color.2
                                    )
                                );
                                println!(
                                    "  {}",
                                    "╚╦╝".truecolor(gen_color.0, gen_color.1, gen_color.2)
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
                                    "║ ╚═╗".truecolor(gen_color.0, gen_color.1, gen_color.2)
                                );
                                for (taskid, task_name, task_status) in &task_list {
                                    if *task_status == TaskStatus::Pending {
                                        tasknum += 1;
                                        println!(
                                            "  {}",
                                            format!(
                                                "║   ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                            )
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                            .bold()
                                        );
                                    }
                                }
                                println!(
                                    "  {}",
                                    "║   ╚══[Pending]"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                            }
                            continue;
                        }
                        '4' => {
                            println!(
                                "{}",
                                "  ╠═╬═[Tasks] All tasks:"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );

                            if !task_list
                                .iter()
                                .any(|(_, _, status)| *status == TaskStatus::Pending)
                            {
                                println!(
                                    "  {}",
                                    "║ ╠══[_] No Pending tasks"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ║".truecolor(gen_color.0, gen_color.1, gen_color.2)
                                );
                            } else {
                                let mut tasknum: u16 = 0;
                                println!(
                                    "  {}",
                                    "║ ╠══[Pending] Pending Tasks: "
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ╠═╗"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                for (taskid, task_name, task_status) in &task_list {
                                    if *task_status == TaskStatus::Pending {
                                        tasknum += 1;
                                        println!(
                                            "  {}",
                                            format!(
                                                "║ ║ ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                            )
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                            .bold()
                                        );
                                    }
                                }
                                println!(
                                    "  {}",
                                    "║ ║ ╚══[Pending]"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ║"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                            }

                            if !task_list
                                .iter()
                                .any(|(_, _, status)| *status == TaskStatus::InProgress)
                            {
                                println!(
                                    "  {}",
                                    "║ ╠══[_] No In-Progress Tasks "
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ║"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                            } else {
                                let mut tasknum: u16 = 0;
                                println!(
                                    "  {}",
                                    "║ ╠══[In-Progress] In-Progress Tasks: "
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ╠═╗"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                for (taskid, task_name, task_status) in &task_list {
                                    if *task_status == TaskStatus::InProgress {
                                        tasknum += 1;
                                        println!(
                                            "  {}",
                                            format!(
                                                "║ ║ ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                            )
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                            .bold()
                                        );
                                    }
                                }
                                println!(
                                    "  {}",
                                    "║ ║ ╚══[In-Progress]"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ║"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                            }
                            if !task_list
                                .iter()
                                .any(|(_, _, status)| *status == TaskStatus::Complete)
                            {
                                println!(
                                    "  {}",
                                    "║ ╠══[_] No Completed tasks"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "╚╦╝"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                continue;
                            } else {
                                let mut tasknum: u16 = 0;
                                println!(
                                    "  {}",
                                    "║ ╠══[Completed] Completed Tasks: "
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                println!(
                                    "  {}",
                                    "║ ╚═╗".truecolor(gen_color.0, gen_color.1, gen_color.2)
                                );
                                for (taskid, task_name, task_status) in &task_list {
                                    if *task_status == TaskStatus::Complete {
                                        tasknum += 1;
                                        println!(
                                            "  {}",
                                            format!(
                                                "║   ╟─Task {tasknum}: {task_name} | ID: {taskid} "
                                            )
                                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                            .bold()
                                        );
                                    }
                                }
                                println!(
                                    "  {}",
                                    "║   ╚══[Completed]"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                            }
                            println!(
                                "  {}",
                                "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                            );
                            println!(
                                "  {}",
                                "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                            );
                            continue;
                        }
                        '5' => {
                            println!(
                                "  {}{}",
                                "╠═╩═[_] "
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold(),
                                "Successfully exited".bright_green().bold()
                            );
                            println!(
                                "  {}",
                                "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                            );
                            break;
                        }
                        _ => {
                            println!("{}", "Invalid choice!\n".red().bold());
                        }
                    }
                }
            }

            '4' => {
                println!(
                    "   {}",
                    "╚╗".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                );
                let mut count = 0;
                loop {
                    if count == 0 {
                        print!(
                            "    {}",
                            "╠══[?] Enter ID of task to edit: "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        count += 1;
                    } else {
                        print!(
                            "    {}",
                            "╠══[?] Enter ID of task to edit: "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                    }

                    let mut task_id = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut task_id)
                        .expect("Failed to read line");
                    let task_id: u16 = match task_id.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!(
                                "    {}",
                                "╠══[!] Invalid input"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );
                            continue;
                        }
                    };
                    if task_id > 1000 {
                        println!(
                            "    {}",
                            "╠══[!] Enter an ID in the form of a positive integer thats below 1001"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        continue;
                    }

                    print!(
                        "    {}",
                        "╠══[?] [Optional] Enter new name of the task: "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    let mut new_task_name = String::new();
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut new_task_name)
                        .expect("Failed to read line");
                    let new_task_name = new_task_name.trim().to_string();
                    println!(
                        "    {}",
                        "║ [1] Pending"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "    {}",
                        "║ [2] Complete"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "    {}",
                        "║ [3] In-Progress"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    println!(
                        "    {}",
                        "║ [4] Unchanged"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    print!(
                        "    {}",
                        "╠══[?] New Task Status > "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    let task_status = input(vec!['1', '2', '3', '4']);
                    let task_status_new = match task_status {
                        '1' => TaskStatus::Pending,
                        '2' => TaskStatus::Complete,
                        '3' => TaskStatus::InProgress,
                        '4' => TaskStatus::Nth,
                        _ => TaskStatus::Nth,
                    };
                    if new_task_name.is_empty() && task_status_new == TaskStatus::Nth {
                        println!(
                            "    {}",
                            "╠══[_] No changes made to the task"
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        print!(
                            "    {}",
                            "╠══[?] Would you like to exit? Y/n > "
                                .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                .bold()
                        );
                        let choice = input(vec!['Y', 'y', 'N', 'n']);
                        match choice {
                            'Y' | 'y' => {
                                println!(
                                    "    {}",
                                    "╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                                break;
                            }
                            'N' | 'n' => {
                                count += 1;
                                println!(
                                    "{}",
                                    "║".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                                );
                                continue;
                            }
                            _ => {
                                println!(
                                    "{}",
                                    "Invalid choice!\n"
                                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                        .bold()
                                );
                                continue;
                            }
                        }
                    }
                    let r1 = File::open(filepath).unwrap();
                    BaseTasks::edit_task(task_id, new_task_name, task_status_new, &r1);
                    drop(r1);
                    if let Ok(sender) = shared_sender.lock() {
                        sender.send(EditStatus::Authorized).unwrap_or_else(|e| {
                            eprintln!("Error sending message: {}", e);
                        });
                    }
                    print!(
                        "    {}",
                        "╠══[?] Would you like to exit? Y/n > "
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    let choice = input(vec!['Y', 'y', 'N', 'n']);
                    match choice {
                        'Y' | 'y' => {
                            println!();
                            break;
                        }
                        'N' | 'n' => {}
                        _ => {
                            println!(
                                "{}",
                                "This was supposed to be unreachable!"
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                                    .bold()
                            );
                            continue;
                        }
                    }
                }
            }

            '5' => {
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
                thread::sleep(Duration::from_millis(700));
                println!(
                    "{}",
                    "  ╔╝"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                continue;
            }

            '6' => {
                let f1 = File::open(filepath).unwrap();
                let tasks = BaseTasks::read_tasks(&f1);
                if tasks.is_empty() {
                    println!(
                        "   {}",
                        "╠═[!] No tasks found"
                            .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            .bold()
                    );
                    thread::sleep(Duration::from_millis(400));
                    println!(
                        "  {}",
                        "╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                    );
                    drop(f1);
                    continue;
                }
                drop(f1);
                let f2 = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(filepath)
                    .unwrap();
                BaseTasks::init(&f2);
                send_edit_status(
                    Arc::clone(&shared_sender),
                    Arc::clone(&status_acknowledged_clone),
                );
                println!(
                    "   {}{}",
                    "╠═[-] "
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold(),
                    "All tasks have been removed".bright_green().bold()
                );
                println!(
                    "  {}",
                    "╔╝".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                );
            }

            '7' => {
                println!(
                    "{}{}{}",
                    "   ╚══[ "
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold(),
                    "Goodbye".green().bold(),
                    " ]".truecolor(gen_color.0, gen_color.1, gen_color.2).bold()
                );
                thread::sleep(Duration::from_secs(1));
                std::process::exit(0);
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
                    "  ╔╝"
                        .truecolor(gen_color.0, gen_color.1, gen_color.2)
                        .bold()
                );
                continue;
            }
        }
    }
}
