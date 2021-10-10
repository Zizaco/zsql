use clap::{AppSettings, Clap};

use zsql::sql_engine::SqlEngine;
use zsql::sql_preproc;

/// Runs SQL queries on csv files
/// Example:
///    zsql "SELECT * from 'my csv file.csv'"
#[derive(Clap)]
#[clap(version = "1.0", author = "Zizaco <zizaco@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Set csv the separator character to be used.
    #[clap(short, default_value = ",")]
    separator: char,
    /// A level of verbosity. Can be used multiple times: -v -vv -vvv
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// SQL query to be executed. Example: "SELECT * from ./path/to/file.csv"
    query: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    // println!("Query: {}", opts.query);

    // match opts.verbose {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     _ => println!("Don't be ridiculous"),
    // }

    run_query(opts.query)
}

fn run_query(query: String) {
    let mut engine = SqlEngine::new(std::io::stdout());
    let (query, files) = sql_preproc::pop_filenames_from_query(&query);
    {
        let files: Vec<&str> = files.iter().map(|s| s as &str).collect();
        engine.load_files(files).unwrap();
    }
    engine.query(&query);
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn run_query_should_output() {
        run_query("SELECT Name,Movie from test/fixtures/oscar_age.csv WHERE Year>2014".to_string());
    }
}
