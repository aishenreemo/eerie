use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParsedArgsError {
    UnknownCommand,
    UnexpectedToken(char),
}

impl fmt::Display for ParsedArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParsedArgsError::*;
        match *self {
            UnknownCommand => write!(f, "Invalid command was provided"),
            UnexpectedToken(c) => write!(f, "Unexpected char while parsing arguments '{c}'"),
        }
    }
}

impl std::error::Error for ParsedArgsError {}

#[derive(Debug)]
pub struct ParsedArgs<'msg> {
    pub command: &'msg str,
    pub positional: HashSet<ParsedArgValue<'msg>>,
    pub flags: HashMap<&'msg str, ParsedArgValue<'msg>>,
    pub switches: HashSet<&'msg str>,
}

impl<'msg> ParsedArgs<'msg> {
    fn init(msg_content: &'msg str) -> Self {
        Self {
            command: msg_content,
            positional: HashSet::new(),
            flags: HashMap::new(),
            switches: HashSet::new(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum ParsedArgValue<'msg> {
    _Val(&'msg str),
}

struct Tokenizer {
    cursor: usize,
    _depth: usize,
    chars: Vec<char>,
}

impl Tokenizer {
    fn init(msg_content: &str) -> Self {
        Tokenizer {
            cursor: 0,
            _depth: 0,
            chars: msg_content.chars().collect(),
        }
    }

    fn is_done(&self, margin: usize) -> bool {
        self.cursor >= (self.chars.len() - margin)
    }
}

pub fn parse_args(msg_content: &'_ str) -> Result<ParsedArgs<'_>, Box<ParsedArgsError>> {
    let mut tokenizer = Tokenizer::init(msg_content);
    let mut output = ParsedArgs::init(msg_content);

    expect_command(msg_content, &mut tokenizer, &mut output)?;
    Ok(output)
}

fn expect_command<'a>(
    msg_content: &'a str,
    tokenizer: &mut Tokenizer,
    output: &mut ParsedArgs<'a>,
) -> Result<(), Box<ParsedArgsError>> {
    use ParsedArgsError::*;
    let start = tokenizer.cursor;
    while !tokenizer.is_done(0) {
        match tokenizer.chars.get(tokenizer.cursor) {
            Some(c) if c.is_whitespace() => break,
            Some(c) if c.is_alphanumeric() => tokenizer.cursor += 1,
            Some(c) => return Err(Box::new(UnexpectedToken(c.to_owned()))),
            None => unreachable!("cursor overflow at expect_command"),
        }
    }
    let end = tokenizer.cursor;

    if start == end {
        return Err(Box::new(UnknownCommand));
    }

    output.command = &msg_content[start..end];
    Ok(())
}
