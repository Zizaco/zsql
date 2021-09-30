use std::fs::File;
use std::io::{self, BufRead};
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct CsvLoader {
    csvs: Vec<Csv>,
}

impl CsvLoader {
    fn new() -> Self {
        Self { csvs: vec![] }
    }

    fn load(&mut self, filepath: &str) -> Result<(), Error> {
        self.csvs.push(Csv::new(filepath)?);

        Ok(())
    }
}

pub struct Csv {
    pub filename: String,
    pub columns: Vec<String>,
    file: File,
}

impl Csv {
    pub fn new(filepath: &str) -> Result<Self, Error> {
        let file = File::open(filepath);

        if let Err(_) = file {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("No such file '{}'", filepath),
            ));
        }

        let file = file.unwrap();
        let filename = String::from(Path::new(filepath).file_name().unwrap().to_string_lossy());
        let columns = Csv::read_headers(&file, &filename)?;

        Ok(Self {
            filename,
            columns,
            file,
        })
    }

    pub fn lines(&self) -> io::Lines<io::BufReader<&File>> {
        io::BufReader::new(&self.file).lines()
    }

    fn read_headers(file: &File, filename: &String) -> Result<Vec<String>, Error> {
        let mut line_reader = io::BufReader::new(file).lines();
        let line = line_reader.next();

        if let None = line {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unable to read contents of '{}'", filename),
            ));
        }

        let sanitize =
            |i: &str| String::from(i.trim().trim_start_matches("\"").trim_end_matches("\""));
        let result = line.unwrap()?.split(',').map(sanitize).collect();
        Ok(result)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn csv_loader_should_load_file() {
        let mut manager = CsvLoader::new();

        manager.load("test/fixtures/oscar_age.csv").unwrap();

        assert_eq!(manager.csvs[0].filename, "oscar_age.csv");
        assert_eq!(
            manager.csvs[0].columns,
            ["Index", "Year", "Age", "Name", "Movie"]
        );
    }

    #[test]
    fn csv_loader_should_return_err_if_file_not_found() {
        let mut manager = CsvLoader::new();

        let result = manager.load("test/fixtures/file_that_doesnt_exists.csv");

        println!("{:?}", result);

        match result {
            Err(err) => {
                assert_eq!(
                    err.to_string(),
                    "No such file 'test/fixtures/file_that_doesnt_exists.csv'"
                );
            }
            Ok(_) => panic!("result is not error"),
        }
    }
}
