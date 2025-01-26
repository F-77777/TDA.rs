pub mod vals {
    pub const FILEPATH: &str = "tasks/tasks.json";
    pub const MAX_TASKS: usize = 1000;
    pub const FOLDER_PATH: &str = "tasks";
    pub const GEN_COLOR: (u8, u8, u8) = (245, 66, 105);

    #[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub enum TaskStatus {
        InProgress,
        Complete,
        Pending,
        Nth,
    }

    #[derive(PartialEq, Debug)]
    pub enum EditStatus {
        Authorized,
        Unauthorized,
    }
}

pub mod functions {
    use super::vals::EditStatus;
    use super::vals::GEN_COLOR as gen_color;
    use colored::Colorize;
    use crossterm::{
        event::{self, Event, KeyCode, KeyEvent},
        terminal::{disable_raw_mode, enable_raw_mode},
    };
    use std::io::{self, Write};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    pub fn input(vec: Vec<char>) -> char {
        while let Err(e) = enable_raw_mode() {
            eprintln!("Failed to enable raw mode: {}", e);
        }
        loop {
            match event::poll(Duration::from_millis(500)) {
                Ok(true) => match event::read() {
                    Ok(Event::Key(KeyEvent {
                        code: KeyCode::Char(c),
                        kind: event::KeyEventKind::Press,
                        ..
                    })) => {
                        if vec.contains(&c) {
                            if let Err(e) = disable_raw_mode() {
                                eprintln!("Failed to disable raw mode: {}", e);
                            }
                            println!(
                                "{}",
                                c.to_string()
                                    .truecolor(gen_color.0, gen_color.1, gen_color.2)
                            );
                            return c;
                        }
                    }
                    Ok(_) => {
                        continue;
                    }
                    Err(e) => {
                        eprintln!("Failed to read event: {}", e);
                    }
                },
                Ok(false) => {
                    if let Err(e) = io::stdout().flush() {
                        eprintln!("Failed to flush stdout: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error during event polling: {}", e);
                }
            }
        }
    }

    pub fn send_edit_status(
        shared_sender: Arc<Mutex<mpsc::Sender<EditStatus>>>,
        status_acknowledged: Arc<AtomicBool>,
    ) {
        while !status_acknowledged.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(10));
        }

        if let Ok(sender) = shared_sender.lock() {
            sender.send(EditStatus::Authorized).unwrap_or_else(|e| {
                eprintln!("Error sending message: {}", e);
            });
            status_acknowledged.store(false, Ordering::SeqCst);
        }
    }
    pub fn display_warning() {
        if let Err(e) = disable_raw_mode() {
            eprintln!("Failed to disable raw mode: {}", e);
        }
        let color = gen_color;
        let message = "Please don't fucking edit the tasks.json file ";
        println!("{}\n", message.truecolor(color.0, color.1, color.2).bold());

        thread::sleep(Duration::from_secs(2));
        std::process::exit(0);
    }
}
