#[macro_use] extern crate lalrpop_util;
mod ast;
mod spans;

fn main() {
    println!("Hello, world!");
}

lalrpop_mod!(pub sz);

#[test]
fn sz_indent() {
    println!("{:?}", sz::IndentParser::new().parse("hello"));

    assert_eq!(sz::IndentParser::new().parse("hello").unwrap().as_str(), "hello");
    assert_eq!(sz::IndentParser::new().parse("hello1").unwrap().as_str(), "hello1");
    assert_eq!(sz::IndentParser::new().parse("hello_1").unwrap().as_str(), "hello_1");
    assert!(sz::IndentParser::new().parse("Hello").is_err());
}

#[test]
fn sz_tag() {
    assert_eq!(sz::TagParser::new().parse("`HELLO").unwrap().as_str(), "`HELLO");
    assert_eq!(sz::TagParser::new().parse("`HELLO1").unwrap().as_str(), "`HELLO1");
    assert!(sz::TagParser::new().parse("HELLO").is_err());
    assert_eq!(sz::TagParser::new().parse("`Hello").unwrap().as_str(), "`Hello");
}

#[test]
fn sz_int_literal() {
    assert_eq!(sz::IntLiteralParser::new().parse("100").unwrap().as_str(), "100");
    assert_eq!(sz::IntLiteralParser::new().parse("-1").unwrap().as_str(), "-1");
    assert_eq!(sz::IntLiteralParser::new().parse("0").unwrap().as_str(), "0");
    assert_eq!(sz::IntLiteralParser::new().parse("-0").unwrap().as_str(), "-0");

    assert!(sz::IntLiteralParser::new().parse("01").is_err());
}

#[test]
fn sz_float_literal() {
    assert_eq!(sz::FloatLiteralParser::new().parse("10.0").unwrap().as_str(), "10.0");
    assert_eq!(sz::FloatLiteralParser::new().parse("-0.1").unwrap().as_str(), "-0.1");
    assert_eq!(sz::FloatLiteralParser::new().parse("1.").unwrap().as_str(), "1.");
    assert!(sz::FloatLiteralParser::new().parse("01").is_err());
    assert!(sz::FloatLiteralParser::new().parse("0.1.1").is_err());
}

#[test]
fn sz_string_literal() {
    assert_eq!(sz::StringLiteralParser::new().parse(r#""abc test""#).unwrap().as_str(), "\"abc test\"");
    assert_eq!(sz::StringLiteralParser::new().parse(r#""abc \test""#).unwrap().as_str(), "\"abc \\test\"");
    assert!(sz::StringLiteralParser::new().parse(r#""abc 
test""#).is_err());
}

#[test]
fn sz_comment() {
    assert!(sz::CommentParser::new().parse(r#"/* test */"#).is_ok());
    assert!(sz::CommentParser::new().parse(r#"/*
line1
test2
    */"#).is_ok());
    assert!(sz::CommentParser::new().parse(r#"// test"#).is_ok());
    assert!(sz::CommentParser::new().parse(r#"/*
line1**
test2**
    */"#).is_err());
}

#[test]
fn sz_script() {
    println!("{:?}", sz::ScriptParser::new().parse(r#""12313";"234""#));
}


#[test]
fn sz_variable() {
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#"null"#));
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#"true"#));
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#"false"#));
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#"test"#));
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#"1.1234"#));
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#"123"#));
    println!("{:?}", sz::VarOrLiteralParser::new().parse(r#""hello world""#));

}
