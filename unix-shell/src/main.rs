mod parser;
mod process;
use std::{env, io::{self, BufRead}};

use crate::parser::Argv;

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if let Some(unit) = Argv::new(line) {
            if unit.program == "exit" {
                break;
            } else if unit.program == "cd" {
                match unit.args.first() {
                    Some(path) => {
                        if let Err(e) = env::set_current_dir(path) {
                            eprintln!("cd: {e}");
                        }
                    }
                    None => eprintln!("cd: missing path"),
                }
            } else {
                process::run(unit);
            }
        }
    }
}
