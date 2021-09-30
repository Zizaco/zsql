use std::io::Error;
use super::csv_loader::CsvLoader;
use rusqlite::{Connection, Result};

pub struct SqlEngine {
    sqlite_conn: Connection,
    csv_loader: CsvLoader
}

impl SqlEngine {
    pub fn new() -> Self {
        Self {
            sqlite_conn: Connection::open_in_memory().unwrap(),
            csv_loader: CsvLoader::new()
        }
    }

    pub fn load_files(&mut self, files: Vec<&str>) -> Result<(), Error> {
        for file in files {
            let csv = self.csv_loader.load(file)?;
            let columns = &csv.columns;
            let filename = &csv.filename;

            let column_list: String = columns.iter().map(|column| format!("{} TEXT", column)).collect::<Vec<String>>().join(&",");

            let create_statement = format!(
                "CREATE TABLE {} ({})",
                filename.trim_end_matches(".csv"),
                column_list
            );

            println!("{}", create_statement);

            self.sqlite_conn.execute(create_statement.as_str(), []).unwrap();


            let sanitize =
            |i: &str| String::from(i.trim().trim_start_matches("\"").trim_end_matches("\""));

            let mut transaction = self.sqlite_conn.transaction().unwrap();

            for line in csv.lines() {
                let line = line.unwrap();

                let insert_statement = format!(
                    "INSERT INTO {} VALUES (\"{}\")",
                    filename.trim_end_matches(".csv"),
                    line.split(',').map(sanitize).collect::<Vec<String>>().join("\", \"")
                );

                println!("{}", insert_statement);

                transaction.execute(insert_statement.as_str(),[]).unwrap();
            }

            transaction.commit().unwrap();

            println!("INSERT COMPLETED");
        }

        Ok(())
    }

    pub fn query(&self, query: &str) {
        let mut stmt = self.sqlite_conn
            .prepare(query)
            .unwrap();

        let mut rows = stmt.query_map([], |row| {
            let line: [String;4] = [
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
            ];
            let line: String = line.join(",");
            Ok(line)
        }).expect("yeah");

        println!("SELECT RESULTS:");
        while let Some(row) = rows.next() {
            println!("{}", row.unwrap());
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_be_instantiated() {
        let engine = SqlEngine::new();
    }

    #[test]
    fn should_load_csv_file() {
        let mut engine = SqlEngine::new();

        engine.load_files(vec!["test/fixtures/oscar_age.csv"]).unwrap();
        engine.query("SELECT * FROM oscar_age");
    }
}
