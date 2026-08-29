mod parser;
mod process;
use std::{
    env,
    io::{self, BufRead},
};

use nix::sys::signal::{signal, SigHandler, Signal};

use crate::parser::parse_line;

fn main() {
    if let Err(e) = unsafe { signal(Signal::SIGINT, SigHandler::SigIgn) } {
        eprintln!("signal: {e}");
    }

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("read: {e}");
                break;
            }
        };

        if let Some(commands) = parse_line(&line) {
            if commands.len() == 1 && commands[0].program == "exit" {
                break;
            } else if commands.len() == 1 && commands[0].program == "cd" {
                match commands[0].args.first() {
                    Some(path) => {
                        if let Err(e) = env::set_current_dir(path) {
                            eprintln!("cd: {e}");
                        }
                    }
                    None => eprintln!("cd: missing path"),
                }
            } else {
                process::run(commands);
            }
        }
    }
}
