use regex::Regex;
use std::path::Path;

pub fn pop_filenames_from_query(query: &str) -> (String, Vec<String>) {
    let mut query = query.to_string();
    let file_regex = Regex::new(r#"(\S+\.csv|'.+csv'|".+csv")"#).unwrap();

    let files: Vec<String> = file_regex.captures_iter(&query)
        .map(|cap| cap[1].trim_matches('"').trim_matches('\'').to_string())
        .collect();

    for file in &files {
        let table_name = Path::new(file).file_name()
            .unwrap().to_string_lossy()
            .to_string();

        let table_name = table_name
            .trim_end_matches(".csv");

        query = query.replace(file, &format!("{}", &table_name));
    }
    let result = query.to_string();
    (result, files)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_pop_filenames_from_query() {
        let (query, files) = pop_filenames_from_query(
            "SELECT * from test/fixtures/oscar_age.csv JOIN './test/foo bar.csv' WHERE Year=2014"
        );

        assert_eq!(query, "SELECT * from oscar_age JOIN 'foo bar' WHERE Year=2014");
        assert_eq!(files, vec!["test/fixtures/oscar_age.csv", "./test/foo bar.csv"]);
    }
}
