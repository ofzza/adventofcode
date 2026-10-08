# 🎄 Advent of Code

My solutions to [Advent of Code](https://adventofcode.com/) puzzles, every December since 2019, in a different language every few years.

![Years](https://img.shields.io/badge/years-2019%E2%80%932025-b31b1b)
![Node.js](https://img.shields.io/badge/Node.js-2019-339933?logo=nodedotjs&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-2020%E2%80%932022-000000?logo=rust&logoColor=white)
![C#](https://img.shields.io/badge/C%23-2023-512BD4?logo=dotnet&logoColor=white)
![Go](https://img.shields.io/badge/Go-2024%E2%80%932025-00ADD8?logo=go&logoColor=white)

## Why this repo exists

- **Team engagement**: every December I use Advent of Code to get the people on my teams solving puzzles, comparing approaches and competing on a private leaderboard. It's an easy way to get people talking about algorithms outside of day-to-day work.
- **Staying sharp**: graphs and path-finding, dynamic programming, interval arithmetic, simulations, number theory, linear algebra. AoC forces me to use the algorithms and data structures that ordinary product work rarely calls for.
- **Learning a new language**: every few years I pick a language I want to get better at and use it for the whole event. 25 puzzles is about the right amount of practice to go from knowing the syntax to writing idiomatic code.
- **Learning how code really runs**: getting the right answer is only half the goal (see below).

## The sub-second challenge

For extra difficulty, I don't stop at getting the correct answer. I aim for every puzzle to **run in under 1 second**. I usually manage it, and sometimes I don't, but the attempt always teaches me something about how code actually runs on real hardware and how to work around common performance problems:

- **Memory**: how often allocations happen, heap vs. stack, reusing buffers instead of allocating new ones
- **CPU**: branch prediction, cache-friendly data layouts, avoiding hidden copies
- **Algorithms**: replacing brute force with the right data structure, memoization, or a closed-form shortcut

The runners are set up for this: `--summary` reports total and per-puzzle execution time, `--repeat N` averages timings over many runs (Rust, C#), and in Go `--graph` charts execution time across all puzzles, so slow puzzles are easy to find. Commits like _"2025-10 optimized"_ are usually where that work happens.

## At a glance

| Years       | Language  | Puzzles                        | Project                             |
| ----------- | --------- | ------------------------------ | ----------------------------------- |
| 2019        | Node.js   | 25 / 25                        | [2019-nodejs](./2019-nodejs/)       |
| 2020 – 2022 | Rust      | 2020\*, 2021, 2022: 25 / 25    | [2020-2022-rust](./2020-2022-rust/) |
| 2023        | C# / .NET | 25 / 25                        | [2023-csharp](./2023-csharp/)       |
| 2024 – 2025 | Go        | 2024: 25 / 25, 2025: 12 / 12   | [2024-2025-go](./2024-2025-go/)     |

<sub>\* 2020 solutions are in the git history.</sub>

## More than a folder of scripts

Each language gets a small puzzle runner, which gets better with every rewrite:

- **Select what to run**: `--year`, `--day`, `--index`, `--tag` (e.g. run only the real solutions, or only the test examples)
- **Inputs**: `--input-file ./input-[:day].txt` with path interpolation, or an inline `--input-value`
- **Validation and benchmarking**: every run is checked against known expected results, and `--repeat N` averages execution time
- **Output**: `--verbose`, `--obfuscate` (to share runs without spoiling answers), `--summary`, `--progress`, and in Go a `--graph` of execution time per puzzle

Solutions aim for general, input-independent approaches. Code that keeps coming back is pulled into shared libraries:

- **Go**: `matrix` (incl. Gaussian elimination), `numbers` (rational / whole-number arithmetic), `pathing`
- **C#**: `Graph`, `Interval`, `Matrix`, `Vector`, `Primes`, `NumericSequence`
- **Rust**: path finding, sparse point clouds, cuboid geometry, and more

## Running

Each project has its own README with the full CLI reference. Quick start:

```sh
cd 2024-2025-go   && go run main.go --year 2025 --tag solution --summary
cd 2023-csharp    && dotnet run -c Release -- --tag solution --summary
cd 2020-2022-rust && cargo run --profile release -- --year 2022
cd 2019-nodejs    && node ./ --year=2019
```

## About

Built and maintained by [ofzza](https://github.com/ofzza). If you're on one of my teams, or just like puzzles, feel free to compare solutions or open a discussion.

Puzzles and inputs © [Eric Wastl / Advent of Code](https://adventofcode.com/about).
