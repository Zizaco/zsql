# `zsql` run SQL queries on csv files

A terminal utility to easily run SQL queries on CSV files. `zsql` is shipped as a _[small single binary](https://github.com/Zizaco/zsql/releases)_ powered by rust and C.

## Key features

```bash
# Select lines from csv
zsql "SELECT * from ./path/to/oscar_age.csv WHERE Year > 2015" > selection.csv

# Select specific columns from csv
zsql "SELECT Age, Name from './name with spaces.csv'" | more

# Join files
zsql "SELECT O.OrderID, C.CustomerName, O.OrderDate
FROM orders.csv AS O
INNER JOIN customers.csv AS C ON O.CustomerID=C.CustomerID"

# Choose separator
zsql -s "|" "SELECT Name, Age from ./separated_by_pipe.csv"

# Runs on windows 🟦
zsql.exe "SELECT COUNT(*) FROM .\file_on_windows.csv"
```

## How to use

```yaml
Runs SQL queries on csv files Example: zsql "SELECT * from 'my csv file.csv'"

USAGE:
    zsql [FLAGS] [OPTIONS] <QUERY>

ARGS:
    <QUERY>    SQL query to be executed.

FLAGS:
    -h, --help       Print help information
    -v, --verbose    A level of verbosity. Can be used multiple times: -v -vv -vvv
    -V, --version    Print version information

OPTIONS:
    -s <SEPARATOR>        Set csv the separator character to be used [default: ,]
```

## Instalation

### Download binaries

Please check out the [Release page](https://github.com/Zizaco/zsql/releases) for prebuilt versions of zsql for **linux**, **mac** and **windows**.


### Build from source

To build zsql from source you need Rust 1.55 or higher.

```bash
make install
```
