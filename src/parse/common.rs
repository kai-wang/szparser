
use nom::{
    sequence::tuple, 
    character::complete::{multispace0, not_line_ending, multispace1, char}, 
    bytes::complete::tag,
    bytes::complete::take_until, branch::alt, multi::many1, 
};

use crate::parse::error::IResult;

pub fn colons(i: &str) -> IResult<&str, ()> {
    let (i, _) = whitespace0(i)?;
    let (i, _) = many1(char(';'))(i)?;
    let (i, _) = whitespace0(i)?;
    Ok((i, ()))
}

pub fn whitespace0(i: &str) -> IResult<&str, ()> {
    let (i, _) = alt((comment, blank))(i)?;
    Ok((i, ()))
}

pub fn whitespace1(i: &str) -> IResult<&str, ()> {
    let (i, _) = alt((comment, space))(i)?;
    Ok((i, ()))
}

pub fn comment(i: &str) -> IResult<&str, ()> {

    let mut parser = tuple((
        multispace0,
        alt((
            multi_line_comment,
            single_line_comment
        )),
        multispace0));

    let (i, _) = parser(i)?;

    Ok((i, ()))
}

fn multi_line_comment(i: &str) -> IResult<&str, ()> {
    let (r, _) = tuple((
        tag("/*"),
        take_until("*/"),
        tag("*/")
    ))(i)?;

    Ok((r, ()))
}

fn single_line_comment(i: &str) -> IResult<&str, ()> {
    let (i, _) = tuple((
        tag("//"),
        not_line_ending
    ))(i)?;

    Ok((i, ()))
}

fn blank(i: &str) -> IResult<&str, ()> {
    let (i, _) = multispace0(i)?;
    Ok((i, ()))
}

fn space(i: &str) -> IResult<&str, ()> {
    let (i, _) = multispace1(i)?;
    Ok((i, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_multiline_comment() {
        let res = comment("/*test*/");
        assert!(res.is_ok());
    }

    #[test]
    fn valid_multiline_comment_spaces() {
        let (remaining, value) = comment(" /* test*/  start").unwrap();
        assert_eq!(remaining, "start");
    }

    #[test]
    fn valid_singleline_comment() {
        let (remaining, value) = comment(" //test test*/ */  start").unwrap();
        assert_eq!(remaining, "");
    }

    #[test]
    fn valid_multiline_comment_with_following_space() {
        let (remaining, value) = whitespace1("/* test*/  start").unwrap();
        assert_eq!(remaining, "start");
    }

    #[test]
    fn invalid_multiline_comment_without_space() {
        assert!(whitespace1("/* test*/start").is_err());
    }

    #[test]
    fn valid_colons() {
        println!("{:?}", colons("/* test*/;;;"));
        assert!(colons("/* test*/;").is_ok());
    }

}