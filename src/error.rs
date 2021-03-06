//! Error handling utilities. WIP.

use std::fmt;
use std::fmt::{Show, Formatter};

use std::io::IoError;

pub type CliError = Box<Error + 'static>;
pub type CliResult<T> = Result<T, CliError>;

pub trait Error {
    fn description(&self) -> &str;

    fn detail(&self) -> Option<&str> { None }
    fn cause(&self) -> Option<&Error> { None }
}

pub trait FromError<E> {
    fn from_err(err: E) -> Self;
}

impl Show for Box<Error + 'static> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E: Error + 'static> FromError<E> for Box<Error + 'static> {
    fn from_err(err: E) -> Box<Error + 'static> {
        box err as Box<Error>
    }
}

impl<'a> Error for &'a str {
    fn description<'a>(&'a self) -> &'a str {
        *self
    }
}

impl Error for String {
    fn description<'a>(&'a self) -> &'a str {
        self[]
    }
}

impl FromError<()> for () {
    fn from_err(error: ()) -> () { () }
}

impl FromError<IoError> for IoError {
    fn from_err(error: IoError) -> IoError { error }
}

impl Error for IoError {
    fn description(&self) -> &str {
        self.desc
    }
    fn detail(&self) -> Option<&str> {
        self.detail.as_ref().map(|s| s[])
    }
}

//fn iter_map_err<T, U, E, I: Iterator<Result<T,E>>>(iter: I,
