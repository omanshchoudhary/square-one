pub struct Argv {
    pub program: String,
    pub args: Vec<String>,
}

impl Argv {
    pub fn new(line: String) -> Option<Self> {
        let mut parts = line.split_whitespace();
        let program = parts.next()?.to_string();
        let args = parts.map(str::to_string).collect();
        Some(Self { program, args })
    }
}
