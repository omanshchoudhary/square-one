# square-one

Small systems programs in Rust, written from first principles. Two crates in one Cargo workspace (edition 2024).

```text
turing-machine/   deterministic single-tape Turing machine
unix-shell/       Unix shell: fork, exec, redirects, pipes
```

```bash
cargo run -p turing-machine -- machine.json [input]
cargo run -p unix-shell
```

## turing-machine

Reads a JSON spec, runs until halt, prints status and the tape (leading/trailing blanks stripped). A second CLI argument overrides `input` in the spec.

```json
{
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
}
```

`move` is `L` or `R`. Optional `max_steps` defaults to `1000000`. Halt statuses: `Accepted`, `Rejected`, `HaltUndefined`, `LimitReached`.

## unix-shell

Interactive loop: read a line, run it, repeat. `exit` or EOF ends the session.

- **Commands** — `fork` → `execvp` → `waitpid`; `PATH` lookup via `execvp`
- **Builtins** — `cd <path>` and `exit` run in the shell process
- **Redirect** — `<`, `>`, `>>` (`open` / `dup2` in the child, before `exec`)
- **Pipes** — `ls | wc -l` (whitespace around `|`)
- **SIGINT** — Ctrl+C stops the child, not the shell

Operators must be separate tokens (`echo hi > out`, not `echo hi>out`). No quoting, globbing, job control, or `&&` / `||`.
