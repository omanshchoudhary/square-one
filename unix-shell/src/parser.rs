pub struct Argv {
    pub program: String,
    pub args: Vec<String>,
    pub stdout: Option<(String, bool)>,
    pub stdin: Option<String>,
}

impl Argv {
    fn from_tokens(tokens: &[&str]) -> Option<Self> {
        let mut words: Vec<String> = Vec::new();
        let mut stdin = None;
        let mut stdout = None;
        let mut tokens = tokens.iter().copied().peekable();

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

pub fn parse_line(line: &str) -> Option<Vec<Argv>> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    let mut commands = Vec::new();
    let mut start = 0;
    for (i, tok) in tokens.iter().enumerate() {
        if *tok == "|" {
            commands.push(Argv::from_tokens(&tokens[start..i])?);
            start = i + 1;
        }
    }
    commands.push(Argv::from_tokens(&tokens[start..])?);
    Some(commands)
}
