use super::csv_loader::CsvLoader;
use rusqlite::{Connection, Result};
use std::io::{Error, Write};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub struct SqlEngine<T: Write> {
    sqlite_conn: Connection,
    csv_loader: CsvLoader,
    output: T,
}

impl<T: Write> SqlEngine<T> {
    pub fn new(output: T) -> Self {
        Self {
            sqlite_conn: Connection::open_in_memory().unwrap(),
            csv_loader: CsvLoader::new(),
            output,
        }
    }

    pub fn load_files(&mut self, files: Vec<&str>) -> Result<(), Error> {
        for file in files {
            let csv = self.csv_loader.load(file)?;
            let columns = &csv.columns;
            let filename = &csv.filename;

            let column_list: String = columns
                .iter()
                .map(|column| format!("{} TEXT", column))
                .collect::<Vec<String>>()
                .join(",");

            let create_statement = format!(
                "CREATE TABLE {} ({})",
                filename.trim_end_matches(".csv"),
                column_list
            );

            // println!("{}", create_statement);

            self.sqlite_conn
                .execute(create_statement.as_str(), [])
                .unwrap();

            let sanitize =
                |i: &str| String::from(i.trim().trim_start_matches('\"').trim_end_matches('\"'));

            let transaction = self.sqlite_conn.transaction().unwrap();

            for line in csv.lines() {
                let line = line.unwrap();

                let insert_statement = format!(
                    "INSERT INTO {} VALUES (\"{}\")",
                    filename.trim_end_matches(".csv"),
                    line.split(',')
                        .map(sanitize)
                        .collect::<Vec<String>>()
                        .join("\", \"")
                );

                // println!("{}", insert_statement);

                transaction.execute(insert_statement.as_str(), []).unwrap();
            }

            transaction.commit().unwrap();

            // println!("INSERT COMPLETED");
        }

        Ok(())
    }

    pub fn query(&mut self, query: &str) {
        let mut stmt = self.sqlite_conn.prepare(query).unwrap();

        let column_count = stmt.column_count();

        let rows = stmt
            .query_map([], |row| {
                let mut line = Vec::<String>::with_capacity(10);
                for i in 0..column_count {
                    line.push(row.get(i).unwrap());
                }
                let line: String = line.join(",");
                Ok(line)
            })
            .expect("yeah");

        // println!("SELECT RESULTS:");
        for row in rows {
            let out = format!("{}{}", row.unwrap(), LINE_ENDING);
            self.output.write_all(out.as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    struct MockWriter {
        pub contents: Option<String>,
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            match std::str::from_utf8(buf) {
                Ok(new_content) => {
                    let new_content = String::from(new_content);
                    let length = new_content.len();
                    self.contents = Some(new_content);

                    Ok(length)
                }
                Err(err) => {
                    let err = std::io::Error::new(std::io::ErrorKind::InvalidData, err);
                    Err(err)
                }
            }
        }

        fn flush(&mut self) -> std::io::Result<()> {
            unimplemented!()
        }
    }

    fn get_writter() -> MockWriter {
        MockWriter { contents: None }
    }

    #[test]
    fn should_load_csv_file_and_query() {
        let mut engine = SqlEngine::new(get_writter());

        engine
            .load_files(vec!["test/fixtures/oscar_age.csv"])
            .unwrap();
        engine.query("SELECT * FROM 'oscar_age' WHERE Year = '2014'");

        let expected = "87,2014,44,Matthew McConaughey,Dallas Buyers Club\n".to_string();
        assert_eq!(engine.output.contents, Some(expected))
    }
}
