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

- Format output:

  | Description | Syntax                              | Explanation                            |
  | ----------- | ----------------------------------- | -------------------------------------- |
  | --verbose   | `$ go run main.go -- --verbose[:N]` | Will output more information           |
  | --obfuscate | `$ go run main.go -- --obfuscate`   | Will obfuscate the final result        |
  | --summary   | `$ go run main.go -- --summary`     | Will print summary at end of execution |

For example:

```sh
go run main.go -- --tag "solution" --verbose --obfuscate
```
