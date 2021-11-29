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

  | Description   | Syntax                                     | Explanation                           |
  | ------------- | ------------------------------------------ | ------------------------------------- |
  | --input-file  | `$ cargo run -- --input-file ./input.txt`  | Path to a file containing input data  |
  | --input-value | `$ cargo run -- --input-value "1 2 3 4 5"` | Explicit string containing input data |

- Format output:

  | Description | Syntax                       | Explanation                     |
  | ----------- | ---------------------------- | ------------------------------- |
  | --verbose   | `$ cargo run -- --verbose`   | Will output more information    |
  | --obfuscate | `$ cargo run -- --obfuscate` | Will obfuscate the final result |

- Result validation:

  | Description | Syntax                                  | Explanation                                                   |
  | ----------- | --------------------------------------- | ------------------------------------------------------------- |
  | --expect    | `$ cargo run -- --expect "Hello world"` | Performs validation to check if result matches expected value |

For example:

```sh
cargo run -- --input-file "./src/year2021/data/demo/input01.txt" --expect "Hello world" --verbose --obfuscate
```
