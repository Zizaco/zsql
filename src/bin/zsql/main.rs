use clap::{AppSettings, Clap};

use zsql::sql_engine::SqlEngine;
use zsql::sql_preproc;

use color_eyre::eyre::Result;

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

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    let verbosity = opts.verbose;
    setup_error_reporting(verbosity)?;

    let result = run_query(opts.query);
    display_error(result, verbosity);

    Ok(())
}

fn run_query(query: String) -> Result<()> {
    let mut engine = SqlEngine::new(std::io::stdout());
    let (query, files) = sql_preproc::pop_filenames_from_query(&query);
    {
        let files: Vec<&str> = files.iter().map(|s| s as &str).collect();
        engine.load_files(files)?
    }
    engine.query(&query)?;
    Ok(())
}

fn setup_error_reporting(verbosity: i32) -> Result<()> {
    color_eyre::install()?;
    if verbosity > 1 && std::env::var("RUST_BACKTRACE").is_err() {
        let backtrace_level = if verbosity > 2 {
            "full".to_string()
        } else {
            verbosity.to_string()
        };
        std::env::set_var("RUST_BACKTRACE", backtrace_level);
    }
    Ok(())
}

fn display_error(result: Result<()>, verbosity: i32) {
    if let Err(err) = result {
        eprintln!("{}", err);
        if verbosity > 1 {
            eprintln!("Error: {:?}", err);
        }
        std::process::exit(1);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn run_query_should_output() {
        run_query("SELECT Name,Movie from test/fixtures/oscar_age.csv WHERE Year>2014".to_string())
            .unwrap();
    }
}
