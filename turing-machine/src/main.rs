mod types;
mod tape;
mod machine;

use std::{env, fs};

use crate::machine::{Machine, MachineSpec};
use crate::tape::Tape;

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);

    let path = args.next().ok_or("usage: turing-machine <machine.json> [input]")?;
    let input_override = args.next();

    let raw = fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let spec: MachineSpec = serde_json::from_str(&raw).map_err(|e| format!("{path}: {e}"))?;

    // CLI top priority then json input
    let input = input_override.unwrap_or_else(|| spec.input.clone());
    let tape = Tape::new(&input, spec.blank);

    let mut machine = Machine::new(spec, tape).map_err(|e| format!("{path}: {e}"))?;

    let status = machine.run();
    let blank = machine.tape().blank();
    println!("{status:?}");
    println!("{}", machine.tape().contents().trim_matches(blank));

    Ok(())
}
