mod parser;
mod process;
use std::io::{self, BufRead};

use crate::parser::Argv;

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if let Some(unit) = Argv::new(line) {
            process::run(unit);
        }
    }
}
