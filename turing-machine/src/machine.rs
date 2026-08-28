use std::collections::{HashMap, HashSet};

use serde::Deserialize;

use crate::tape::Tape;
use crate::types::{Direction, ExecutionStatus};

#[derive(Debug, Deserialize)]
pub struct MachineSpec {
    pub states: HashSet<String>,
    pub start: String,
    pub accept_states: HashSet<String>,
    pub reject_states: HashSet<String>,
    pub alphabet: HashSet<char>,
    pub tape_symbols: HashSet<char>,
    pub blank: char,
    pub input: String,
    #[serde(default = "default_max_steps")]
    pub max_steps: u64,
    pub transitions: Vec<Rule>,
}

fn default_max_steps() -> u64 {
    1_000_000
}

#[derive(Debug, Deserialize)]
pub struct Rule {
    pub state: String,
    pub read: char,
    pub next: String,
    pub write: char,
    #[serde(rename = "move")]
    pub direction: Direction,
}

impl MachineSpec {
    pub fn validate(&self) -> Result<(), String> {
        if !self.states.contains(&self.start) {
            return Err(format!("start state `{}` is not in states", self.start));
        }
        if !self.tape_symbols.contains(&self.blank) {
            return Err(format!("blank `{}` is not in tape_symbols", self.blank));
        }

        for symbol in &self.alphabet {
            if !self.tape_symbols.contains(symbol) {
                return Err(format!("alphabet symbol `{symbol}` is not in tape_symbols"));
            }
        }

        for (label, set) in [
            ("accept", &self.accept_states),
            ("reject", &self.reject_states),
        ] {
            for state in set {
                if !self.states.contains(state) {
                    return Err(format!("{label} state `{state}` is not in states"));
                }
            }
        }

        for rule in &self.transitions {
            for (label, state) in [("state", &rule.state), ("next", &rule.next)] {
                if !self.states.contains(state) {
                    return Err(format!(
                        "transition ({}, {}): {label} `{state}` is not in states",
                        rule.state, rule.read
                    ));
                }
            }
            for (label, symbol) in [("read", rule.read), ("write", rule.write)] {
                if !self.tape_symbols.contains(&symbol) {
                    return Err(format!(
                        "transition ({}, {}): {label} `{symbol}` is not in tape_symbols",
                        rule.state, rule.read
                    ));
                }
            }
        }

        Ok(())
    }
}

pub struct Machine {
    transition: HashMap<(String, char), (String, char, Direction)>,
    status: ExecutionStatus,
    accept_states: HashSet<String>,
    reject_states: HashSet<String>,
    current: String,
    max_steps: u64,
    tape: Tape,
}

impl Machine {
    pub fn new(spec: MachineSpec, tape: Tape) -> Result<Self, String> {
        spec.validate()?;

        let transition = spec
            .transitions
            .into_iter()
            .map(|r| ((r.state, r.read), (r.next, r.write, r.direction)))
            .collect();

        Ok(Self {
            transition,
            status: ExecutionStatus::Running,
            accept_states: spec.accept_states,
            reject_states: spec.reject_states,
            current: spec.start,
            max_steps: spec.max_steps,
            tape,
        })
    }

    pub fn tape(&self) -> &Tape {
        &self.tape
    }

    pub fn run(&mut self) -> ExecutionStatus {
        let mut steps = 0;
        while !(self.accept_states.contains(&self.current)
            || self.reject_states.contains(&self.current))
        {
            if steps == self.max_steps {
                self.status = ExecutionStatus::LimitReached;
                return self.status;
            }
            steps += 1;

            let symbol = self.tape.read();
            let key = (self.current.clone(), symbol);

            // No rule for this, hence machine stuck
            let Some((next_state, next_symbol, direction)) = self.transition.get(&key).cloned()
            else {
                self.status = ExecutionStatus::HaltUndefined;
                return self.status;
            };

            self.tape.write(next_symbol);
            self.tape.move_head(direction);
            self.current = next_state;
        }

        self.status = if self.accept_states.contains(&self.current) {
            ExecutionStatus::Accepted
        } else {
            ExecutionStatus::Rejected
        };
        self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLIP: &str = r#"{
      "states": ["q0", "qA"],
      "start": "q0",
      "accept_states": ["qA"],
      "reject_states": [],
      "alphabet": ["0", "1"],
      "tape_symbols": ["0", "1", "_"],
      "blank": "_",
      "input": "101",
      "transitions": [
        { "state": "q0", "read": "0", "next": "q0", "write": "1", "move": "R" },
        { "state": "q0", "read": "1", "next": "q0", "write": "0", "move": "R" },
        { "state": "q0", "read": "_", "next": "qA", "write": "_", "move": "L" }
      ]
    }"#;

    const REJECT_ON_ONE: &str = r#"{
      "states": ["q0", "qA", "qR"],
      "start": "q0",
      "accept_states": ["qA"],
      "reject_states": ["qR"],
      "alphabet": ["0", "1"],
      "tape_symbols": ["0", "1", "_"],
      "blank": "_",
      "input": "",
      "transitions": [
        { "state": "q0", "read": "0", "next": "q0", "write": "0", "move": "R" },
        { "state": "q0", "read": "1", "next": "qR", "write": "1", "move": "R" },
        { "state": "q0", "read": "_", "next": "qA", "write": "_", "move": "L" }
      ]
    }"#;

    const NEVER_HALTS: &str = r#"{
      "states": ["q0", "qA"],
      "start": "q0",
      "accept_states": ["qA"],
      "reject_states": [],
      "alphabet": ["0"],
      "tape_symbols": ["0", "_"],
      "blank": "_",
      "input": "",
      "max_steps": 500,
      "transitions": [
        { "state": "q0", "read": "0", "next": "q0", "write": "0", "move": "R" },
        { "state": "q0", "read": "_", "next": "q0", "write": "_", "move": "R" }
      ]
    }"#;

    fn run(json: &str, input: &str) -> (ExecutionStatus, String) {
        let spec: MachineSpec = serde_json::from_str(json).unwrap();
        let blank = spec.blank;
        let tape = Tape::new(input, blank);
        let mut machine = Machine::new(spec, tape).unwrap();
        let status = machine.run();
        let contents = machine.tape.contents().trim_matches(blank).to_string();
        (status, contents)
    }

    #[test]
    fn flips_every_bit() {
        let (status, tape) = run(FLIP, "101");
        assert_eq!(status, ExecutionStatus::Accepted);
        assert_eq!(tape, "010");
    }

    #[test]
    fn flips_empty_input() {
        let (status, tape) = run(FLIP, "");
        assert_eq!(status, ExecutionStatus::Accepted);
        assert_eq!(tape, "");
    }

    #[test]
    fn halts_when_no_rule_matches() {
        let (status, _) = run(FLIP, "2");
        assert_eq!(status, ExecutionStatus::HaltUndefined);
    }

    #[test]
    fn reaches_reject_state() {
        let (status, _) = run(REJECT_ON_ONE, "001");
        assert_eq!(status, ExecutionStatus::Rejected);
    }

    #[test]
    fn accepts_when_reject_state_unreached() {
        let (status, _) = run(REJECT_ON_ONE, "000");
        assert_eq!(status, ExecutionStatus::Accepted);
    }

    #[test]
    fn stops_at_step_limit() {
        let (status, _) = run(NEVER_HALTS, "");
        assert_eq!(status, ExecutionStatus::LimitReached);
    }

    #[test]
    fn max_steps_defaults_when_absent() {
        let spec: MachineSpec = serde_json::from_str(FLIP).unwrap();
        assert_eq!(spec.max_steps, 1_000_000);
    }

    #[test]
    fn rejects_unknown_state_in_transition() {
        let bad = FLIP.replace("\"next\": \"qA\"", "\"next\": \"qTypo\"");
        let spec: MachineSpec = serde_json::from_str(&bad).unwrap();
        let err = Machine::new(spec, Tape::new("101", '_')).err().expect("spec should be rejected");
        assert!(err.contains("qTypo"), "{err}");
    }

    #[test]
    fn rejects_symbol_outside_tape_symbols() {
        let bad = FLIP.replace("\"write\": \"1\"", "\"write\": \"z\"");
        let spec: MachineSpec = serde_json::from_str(&bad).unwrap();
        let err = Machine::new(spec, Tape::new("101", '_')).err().expect("spec should be rejected");
        assert!(err.contains('z'), "{err}");
    }

    #[test]
    fn starts_in_start_state() {
        let spec: MachineSpec = serde_json::from_str(FLIP).unwrap();
        let machine = Machine::new(spec, Tape::new("101", '_')).unwrap();
        assert_eq!(machine.current, "q0");
        assert_eq!(machine.status, ExecutionStatus::Running);
    }
}
