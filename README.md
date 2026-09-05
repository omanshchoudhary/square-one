# square-one

Small systems programs written from first principles. Two Rust crates in one Cargo workspace (edition 2024), plus a standalone C++ project.

```text
turing-machine/   deterministic single-tape Turing machine
unix-shell/       Unix shell: fork, exec, redirects, pipes
job-scheduler/    CPU scheduling algorithms (C++)
```

```bash
cargo run -p turing-machine -- machine.json [input]
cargo run -p unix-shell

g++ -std=c++17 -o job-scheduler/js job-scheduler/src/*.cpp job-scheduler/src/schedulers/*.cpp
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

## job-scheduler

Reads a workload on stdin, runs one scheduling algorithm, prints the result table. Input is assumed valid.

```text
1  FCFS                  non-preemptive, arrival order
2  SJF                   non-preemptive, shortest burst first
3  SRTF                  preemptive, shortest remaining time first
4  RR                    preemptive, fixed time quantum
5  Preemptive Priority   lower number wins
```

First line is the algorithm, then the process count, then one line per process. Ties break on `pid`.

```text
<algorithm>
<n>
<pid> <at> <bt>            # 1, 2, 3, 4
<pid> <at> <bt> <priority> # 5
<quantum>                  # 4 only, after the process lines
```

Output is one line per process: `pid at bt ct tat wt`.

```console
$ printf '4\n4\n1 0 5\n2 1 3\n3 2 8\n4 3 6\n2\n' | ./job-scheduler/js
1 0 5 14 14 9
2 1 3 11 10 7
3 2 8 22 20 12
4 3 6 20 17 11
```

`fcfs` and `rr` sort the process list, so their rows come out in arrival order; the other three preserve input order.
