# adventofcode

### Run

Run puzzles using:

```sh
$ go run main.go -- [arg1 [arg2 ...]]
```

Available arguments:

- Select which puzzle(s) to run:

  | Description | Syntax                               | Explanation                                         |
  | ----------- | ------------------------------------ | --------------------------------------------------- |
  | --year      | `$ go run main.go -- --year 2024`    | Will run only puzzles marked as `year: 2024`        |
  | --day       | `$ go run main.go -- --day 3`        | Will run only puzzles marked as `day: 3` of a year  |
  | --index     | `$ go run main.go -- --index 1`      | Will run only puzzles marked with `index: 1`        |
  | --tag       | `$ go run main.go -- --tag solution` | Will run only puzzles marked with `tag: "solution"` |

- Specify input data

  | Description   | Syntax                                                | Explanation                                        |
  | ------------- | ----------------------------------------------------- | -------------------------------------------------- |
  | --input-file  | `$ go run main.go -- --input-file ./input.txt`        | Path to a file containing input data               |
  |               | `$ go run main.go -- --input-file ./input-[:day].txt` | Interpolated path to a file containing input data  |
  |               |                                                       | Allowed: `[:year]`, `[:day]`, `[:index]`, `[:tag]` |
  | --input-value | `$ go run main.go -- --input-value "1 2 3 4 5"`       | Explicit string containing input data              |

- Result validation:

  | Description | Syntax                                       | Explanation                                                    |
  | ----------- | -------------------------------------------- | -------------------------------------------------------------- |
  | --expect    | `$ go run main.go -- --expect "Hello world"` | Performs validation to check if result matches expected value  |
  | --repeat    | `$ go run main.go -- --repeat 1000`          | Runs each puzzle 1000 times to get more precise average timing |

- Format output:

  | Description   | Syntax                              | Explanation                                                                                              |
  | ------------- | ----------------------------------- | -------------------------------------------------------------------------------------------------------- |
  | --verbose[:N] | `$ go run main.go -- --verbose[:N]` | Will output more information (N is an optional logging level: 0 No logging, 1 Reduced, 2 Verbose, 9 All) |
  | --obfuscate   | `$ go run main.go -- --obfuscate`   | Will obfuscate the final result                                                                          |
  | --progress    | `$ go run main.go -- --progress`    | Will show progress bar while executing                                                                   |
  | --summary     | `$ go run main.go -- --summary`     | Will print summary at end of execution                                                                   |

For example:

```sh
go run main.go -- --tag "demo" --expect "Hello world" --verbose --obfuscate
```
