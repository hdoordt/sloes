use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    NotADomain
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl error::Error for Error {}
