pub struct Argv {
    pub program: String,
    pub args: Vec<String>,
    pub stdout: Option<(String, bool)>,
    pub stdin: Option<String>,
}

impl Argv {
    pub fn new(line: String) -> Option<Self> {
        let mut tokens = line.split_whitespace().peekable();
        let mut words: Vec<String> = Vec::new();
        let mut stdin = None;
        let mut stdout = None;

        while let Some(tok) = tokens.next() {
            match tok {
                "<" | ">" | ">>" => {
                    let path = tokens.next()?.to_string();
                    match tok {
                        "<" => stdin = Some(path),
                        ">" => stdout = Some((path, false)),
                        ">>" => stdout = Some((path, true)),
                        _ => unreachable!(),
                    }
                }
                _ => words.push(tok.to_string()),
            }
        }

        let mut words = words.into_iter();
        let program = words.next()?;
        let args = words.collect();
        Some(Self {
            program,
            args,
            stdout,
            stdin,
        })
    }
}
