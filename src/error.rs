use peg::error::ParseError;
use peg::str::LineCol;

pub struct CodeGenError {}
#[derive(Debug)]
pub enum CompileError {
    ParseError(ParseError<LineCol>),
    IOError(std::io::Error),
    CodeGenError(String),
    SyntaxError(String),
    RuntimeError(String, Option<i32>),
}
