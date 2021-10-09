#![allow(dead_code)]

pub mod csv_loader;
pub mod sql_engine;
pub mod sql_preproc;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
