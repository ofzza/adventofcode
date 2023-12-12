# adventofcode

### Run

Run puzzles using:

```sh
$ dotnet run -- [arg1 [arg2 ...]]
```

Available arguments:

- Select which puzzle(s) to run:

  | Description | Syntax                           | Explanation                                         |
  | ----------- | -------------------------------- | --------------------------------------------------- |
  | --year      | `$ dotnet run -- --year 2021`    | Will run only puzzles marked as `year: 1`           |
  | --day       | `$ dotnet run -- --day 3`        | Will run only puzzles marked as `day: 3` of a year  |
  | --index     | `$ dotnet run -- --index 1`      | Will run only puzzles marked with `index: 1`        |
  | --tag       | `$ dotnet run -- --tag solution` | Will run only puzzles marked with `tag: "solution"` |

- Specify input data

  | Description   | Syntax                                            | Explanation                                        |
  | ------------- | ------------------------------------------------- | -------------------------------------------------- |
  | --input-file  | `$ dotnet run -- --input-file ./input.txt`        | Path to a file containing input data               |
  |               | `$ dotnet run -- --input-file ./input-[:day].txt` | Interpolated path to a file containing input data  |
  |               |                                                   | Allowed: `[:year]`, `[:day]`, `[:index]`, `[:tag]` |
  | --input-value | `$ dotnet run -- --input-value "1 2 3 4 5"`       | Explicit string containing input data              |

- Result validation:

  | Description | Syntax                                   | Explanation                                                    |
  | ----------- | ---------------------------------------- | -------------------------------------------------------------- |
  | --expect    | `$ dotnet run -- --expect "Hello world"` | Performs validation to check if result matches expected value  |
  | --repeat    | `$ dotnet run -- --repeat 1000`          | Runs each puzzle 1000 times to get more precise average timing |

- Format output:

  | Description   | Syntax                          | Explanation                                                                                              |
  | ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------- |
  | --verbose[:N] | `$ dotnet run -- --verbose[:N]` | Will output more information (N is an optional logging level: 0 No logging, 1 Reduced, 2 Verbose, 9 All) |
  | --obfuscate   | `$ dotnet run -- --obfuscate`   | Will obfuscate the final result                                                                          |
  | --progress    | `$ dotnet run -- --progress`    | Will show progress bar while executing                                                                   |
  | --summary     | `$ dotnet run -- --summary`     | Will print summary at end of execution                                                                   |

For example:

```sh
dotnet run -- --tag "demo" --expect "Hello world" --verbose --obfuscate
```

... or to run a release version with compiler optimizations enabled:

```sh
dotnet run -c Release -- --tag "demo" --expect "Hello world" --verbose --obfuscate --progress
```
