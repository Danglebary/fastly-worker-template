use std::{error::Error, fmt};

#[derive(Debug)]
pub struct UnexpectedMethodError;

impl Error for UnexpectedMethodError {}

impl fmt::Display for UnexpectedMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected method recieved")
    }
}

#[derive(Debug)]
pub struct UnknownRouteError;

impl Error for UnknownRouteError {}

impl fmt::Display for UnknownRouteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown route requested")
    }
}
