use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct OccupiedError {

}

impl fmt::Display for OccupiedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The given position was occupied").unwrap();
        Ok(())
    }
}

impl Error for OccupiedError {

}

impl OccupiedError {
    pub fn new() -> Self {
        Self { }
    }
}