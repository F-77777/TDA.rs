pub mod functions {
    use super::vals::GEN_COLOR as gen_color;
    use colored::Colorize;
    use crossterm::{
        event::{self, Event, KeyCode, KeyEvent},
        terminal::{disable_raw_mode, enable_raw_mode},
    };
    use std::io::{self, Write};
    pub fn input(vec: Vec<char>) -> char {
        // Enable raw mode and handle any potential errors
        loop {
            if let Err(e) = enable_raw_mode() {
                eprintln!("Failed to enable raw mode: {}", e);
            } else {
                break;
            }
        }
        loop {
            match event::poll(std::time::Duration::from_millis(500)) {
                Ok(true) => {
                    // If an event is available, try to read it
                    match event::read() {
                        Ok(Event::Key(KeyEvent { code, .. })) => match code {
                            KeyCode::Char(c) => {
                                if vec.iter().any(|x| x == &c) {
                                    if let Err(e) = disable_raw_mode() {
                                        eprintln!("Failed to disable raw mode: {}", e);
                                    }
                                    print!(
                                        "{}",
                                        c.to_string().truecolor(
                                            gen_color.0,
                                            gen_color.1,
                                            gen_color.2
                                        )
                                    );
                                    return c;
                                }
                            }
                            _ => continue,
                        },
                        Ok(_) => {
                            continue;
                        } // Ignore non-key events
                        Err(e) => {
                            eprintln!("Failed to read event: {}", e);
                            continue; // Exit the loop on error
                        }
                    }
                }
                Ok(false) => {
                    // No event within the polling duration
                    if let Err(e) = io::stdout().flush() {
                        eprintln!("Failed to flush stdout: {}", e);
                        continue; // Exit the loop on error
                    }
                }
                Err(e) => {
                    // Handle poll errors
                    eprintln!("Error during event polling: {}", e);
                    continue;
                }
            }
        }
    }
    pub fn is_valid_status(input: &String) -> bool {
        input.as_str().eq_ignore_ascii_case("pending")
            || input.eq_ignore_ascii_case("in progress")
            || input.eq_ignore_ascii_case("completed")
            || input.eq_ignore_ascii_case("in-progress")
            || input.eq_ignore_ascii_case("complete")
    }
}
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
    }

    #[derive(PartialEq, Debug)]
    pub enum EditStatus {
        Authorized,
        Unauthorized,
    }

    pub enum StatusCode {
        Break,
    }
}
