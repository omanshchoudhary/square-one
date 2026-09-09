# square-one

Systems and distributed systems, built from first principles. Two Rust crates in one Cargo workspace (edition 2024), plus three standalone C++ projects.

```text
bloom-filter/        probabilistic set membership (C++)
consistent-hashing/  hash ring with virtual nodes (C++)
job-scheduler/       CPU scheduling algorithms (C++)
unix-shell/          Unix shell: fork, exec, redirects, pipes
turing-machine/      deterministic single-tape Turing machine
```

```bash
cd bloom-filter && make && ./bloom
cd consistent-hashing && make && ./ring

g++ -std=c++17 -o job-scheduler/js job-scheduler/src/*.cpp job-scheduler/src/schedulers/*.cpp

cargo run -p unix-shell
cargo run -p turing-machine -- machine.json [input]
```

## bloom-filter

Answers "have I seen this key?" in a fraction of the memory a real set would need. Never a false negative; false positives happen at a rate you choose up front. `make && ./bloom` runs the tests and both reports.

- **Bit array** — `vector<uint64_t>`, bit `i` living at position `i % 64` of word `i / 64`.
- **Two hashes, k indices** — the k-th bit is `(h1 + k·h2) % m`, so two hashes stand in for k of them. `hash1` and `hash2` are the same FNV-1a loop with different starting values, each finished with a splitmix64 round. `hash2` is forced odd so it can never be a multiple of `m` — that would collapse all k indices onto one bit.
- **Sizing** — give it an item count and a target false-positive rate; it derives `m = -n·ln(p)/(ln2)²` and `k = (m/n)·ln2` itself, rounding `m` up to a multiple of 64.

You ask for a rate, and you get it:

```console
false positive rate, 100000 keys inserted, 1000000 queried
  target         bits   k   predicted  measured       fill   theory
  0.1          479296   3    10.0694%  10.0291%     0.4652   0.4652
  0.01         958528   7     1.0038%   1.0048%     0.5188   0.5182
  0.001       1437760  10     0.1000%   0.0989%     0.5014   0.5012
  0.0001      1917056  13     0.0100%   0.0108%     0.4926   0.4924
```

The fill column sits near 0.5 on every row — optimal `k` is exactly the value that drives the array half full, which is where each bit carries the most information.

False-positive rate is a U-curve in `k`: too few hashes and you don't discriminate, too many and you saturate the array.

```console
k sweep, m=958528 fixed, 100000 keys inserted, 1000000 queried
  k=1     9.9029%   9.9069%  ###########################################################
  k=5     1.0997%   1.1093%  #######
  k=6     1.0038%   1.0142%  ######  <- lowest
  k=7     1.0048%   1.0038%  ######
  k=8     1.0525%   1.0526%  ######
  k=15    3.0211%   2.9631%  ##################

  formula predicts k=7, true optimum (m/n)ln2 = 6.644
  measured minimum at k=6, +/- 0.0195% at 95% confidence
```

k=6 and k=7 differ by 0.001 points against a ±0.0195% band, so they're tied — the true optimum is 6.644, sitting between them. The curve is flat at the bottom, which is why nobody tunes `k` by ±1.

Delete isn't supported: you can't unset a bit, because other keys may depend on it.

## consistent-hashing

Maps keys to nodes so that adding or removing a node moves only that node's share of keys, instead of remapping everything. `make && ./ring` builds and prints three reports over 100k keys.

- **Hash** — FNV-1a, then a splitmix64 finalizer. FNV-1a alone has weak avalanche, so `node0#0`, `node0#1`, … land numerically adjacent and every virtual node clumps together. The finalizer is what makes the ring spread.
- **Ring** — `map<uint64_t, string>` of hash point → node. Lookup is `lower_bound` on the key's hash, wrapping to the first point when the key hashes past the last one.
- **Virtual nodes** — each node occupies `vnodes` points, placed by hashing `name$i`. Per-node rather than ring-wide, so a bigger node can simply take more points.

Removing a node moves only its own keys — verified at 0 keys displaced from the untouched nodes.

```console
distribution, 10 nodes / 100000 keys
  vnodes=  1  stddev= 7209.5 ( 72.1%)  min= 1149  max=23709
  vnodes= 10  stddev= 3368.1 ( 33.7%)  min= 5739  max=18423
  vnodes=100  stddev= 1227.2 ( 12.3%)  min= 8084  max=12748
  vnodes=500  stddev=  422.6 (  4.2%)  min= 9187  max=10700
```

Load imbalance falls as `1/√vnodes`, which is why real systems settle near 100–200 rather than pushing higher.

```console
adding a 6th node to 5
  consistent hashing   18062 / 100000  (18.1%)   theory 1/6 = 16.7%
  hash(key) % N        83509 / 100000  (83.5%)
```

That gap is the whole point of the algorithm.

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

## unix-shell

Interactive loop: read a line, run it, repeat. `exit` or EOF ends the session.

- **Commands** — `fork` → `execvp` → `waitpid`; `PATH` lookup via `execvp`
- **Builtins** — `cd <path>` and `exit` run in the shell process
- **Redirect** — `<`, `>`, `>>` (`open` / `dup2` in the child, before `exec`)
- **Pipes** — `ls | wc -l` (whitespace around `|`)
- **SIGINT** — Ctrl+C stops the child, not the shell

Operators must be separate tokens (`echo hi > out`, not `echo hi>out`). No quoting, globbing, job control, or `&&` / `||`.

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
