
use thiserror::Error;
use nom::Err;
use nom::error::{ErrorKind, ParseError};

#[derive(Debug, Error)]
pub enum Error<T> {
    ParserError(T)
}

pub type IResult<I, O, E = Error<I>> = Result<(I, O), Err<E>>;

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, _: ErrorKind) -> Self {
        Self::ParserError(input)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}