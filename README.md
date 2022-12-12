# adventofcode

### Run

Run puzzles using:

```sh
$ cargo run [-- [arg1 [arg2 ...]]]
```

Available arguments:

- Select which puzzle(s) to run:

  | Description | Syntax                          | Explanation                                         |
  | ----------- | ------------------------------- | --------------------------------------------------- |
  | --year      | `$ cargo run -- --year 2021`    | Will run only puzzles marked as `year: 1`           |
  | --day       | `$ cargo run -- --day 3`        | Will run only puzzles marked as `day: 3` of a year  |
  | --index     | `$ cargo run -- --index 1`      | Will run only puzzles marked with `index: 1`        |
  | --tag       | `$ cargo run -- --tag solution` | Will run only puzzles marked with `tag: "solution"` |

- Specify input data:

  | Description   | Syntax                                           | Explanation                                        |
  | ------------- | ------------------------------------------------ | -------------------------------------------------- |
  | --input-file  | `$ cargo run -- --input-file ./input.txt`        | Path to a file containing input data               |
  |               | `$ cargo run -- --input-file ./input-[:day].txt` | Interpolated path to a file containing input data  |
  |               |                                                  | Allowed: `[:year]`, `[:day]`, `[:index]`, `[:tag]` |
  | --input-value | `$ cargo run -- --input-value "1 2 3 4 5"`       | Explicit string containing input data              |

- Result validation:

  | Description | Syntax                                  | Explanation                                                    |
  | ----------- | --------------------------------------- | -------------------------------------------------------------- |
  | --expect    | `$ cargo run -- --expect "Hello world"` | Performs validation to check if result matches expected value  |
  | --repeat    | `$ cargo run -- --repeat 1000`          | Runs each puzzle 1000 times to get more precise average timing |

- Format output:

  | Description | Syntax                       | Explanation                     |
  | ----------- | ---------------------------- | ------------------------------- |
  | --verbose   | `$ cargo run -- --verbose`   | Will output more information    |
  | --obfuscate | `$ cargo run -- --obfuscate` | Will obfuscate the final result |

For example:

```sh
cargo run -- --tag "demo" --expect "Hello world" --verbose --obfuscate
```

... or to run a release version with compiler optimizations enabled:

```sh
cargo run --profile release -- --tag "demo" --expect "Hello world" --verbose --obfuscate
```
