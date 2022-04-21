use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ParsedArgsError {
    UnknownCommand,
    UnexpectedToken(char),
    ExpectedWhitespace,
    ExpectedFlagKeyPrefix,
    ExpectedClosingDelimiter,
}

impl fmt::Display for ParsedArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ParsedArgsError::*;
        match *self {
            UnknownCommand => write!(f, "Invalid command was provided"),
            UnexpectedToken(c) => write!(f, "Unexpected char while parsing arguments '{c}'"),
            ExpectedWhitespace => write!(f, "Expected a whitespace."),
            ExpectedFlagKeyPrefix => write!(f, "Expected a flag prefix."),
            ExpectedClosingDelimiter => write!(f, "Expected a closing delimiter."),
        }
    }
}

impl std::error::Error for ParsedArgsError {}

#[derive(Debug)]
pub struct ParsedArgs<'msg> {
    pub command: &'msg str,
    pub positional: Vec<&'msg str>,
    pub flags: HashMap<&'msg str, &'msg str>,
    pub switches: HashSet<&'msg str>,
}

impl<'msg> ParsedArgs<'msg> {
    fn init(msg_content: &'msg str) -> Self {
        Self {
            command: msg_content,
            positional: Vec::new(),
            flags: HashMap::new(),
            switches: HashSet::new(),
        }
    }
}

#[derive(Debug)]
struct Tokenizer {
    cursor: usize,
    depth: usize,
    chars: Vec<char>,
}

impl Tokenizer {
    fn init(msg_content: &str) -> Self {
        Tokenizer {
            cursor: 0,
            depth: 0,
            chars: msg_content.chars().collect(),
        }
    }

    fn is_done(&self, margin: usize) -> bool {
        self.cursor >= (self.chars.len() - margin)
    }
}

pub fn parse_args(msg_content: &'_ str) -> ParsedArgs<'_> {
    let mut tokenizer = Tokenizer::init(msg_content);
    let mut output = ParsedArgs::init(msg_content);

    expect_command(msg_content, &mut tokenizer, &mut output).ok();
    while !tokenizer.is_done(0) {
        if let Err(_) = expect_arg(msg_content, &mut tokenizer, &mut output) {
            break;
        }
    }

    output
}

fn expect_whitespace(tokenizer: &mut Tokenizer) -> Result<(), Box<ParsedArgsError>> {
    let start = tokenizer.cursor;
    while !tokenizer.is_done(0) {
        match tokenizer.chars.get(tokenizer.cursor) {
            Some(c) if c.is_whitespace() => tokenizer.cursor += 1,
            Some(_) => break,
            None => unreachable!("cursor overflow at expect_whitespace"),
        }
    }
    let end = tokenizer.cursor;

    if start == end {
        return Err(Box::new(ParsedArgsError::ExpectedWhitespace));
    }

    Ok(())
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

fn expect_arg<'a>(
    msg_content: &'a str,
    tokenizer: &mut Tokenizer,
    output: &mut ParsedArgs<'a>,
) -> Result<(), Box<ParsedArgsError>> {
    let flag_key_prefix = "-".repeat(tokenizer.depth + 1);
    while !tokenizer.is_done(0) {
        let slice = &msg_content[tokenizer.cursor..];
        if slice.trim().is_empty() {
            return Ok(());
        }
        expect_whitespace(tokenizer)?;
        if slice.trim().starts_with(&flag_key_prefix) {
            expect_flag_key(msg_content, tokenizer, output)?;
        } else {
            expect_positional_arg(msg_content, tokenizer, output)?;
        }
    }

    Ok(())
}

fn expect_flag_key<'a>(
    msg_content: &'a str,
    tokenizer: &mut Tokenizer,
    output: &mut ParsedArgs<'a>,
) -> Result<(), Box<ParsedArgsError>> {
    use ParsedArgsError::UnexpectedToken;

    let slice = &msg_content[tokenizer.cursor..];
    let flag_key_prefix = "-".repeat(tokenizer.depth + 1);

    if !slice.starts_with(&flag_key_prefix) {
        return Err(Box::new(ParsedArgsError::ExpectedFlagKeyPrefix));
    }

    tokenizer.cursor += tokenizer.depth + 1;
    let start = tokenizer.cursor;
    while !tokenizer.is_done(0) {
        match tokenizer.chars.get(tokenizer.cursor) {
            Some(c) if c.is_whitespace() => break,
            Some(c) if c.is_alphanumeric() => tokenizer.cursor += 1,
            Some(&'-') => tokenizer.cursor += 1,
            Some(c) => return Err(Box::new(UnexpectedToken(c.to_owned()))),
            None => unreachable!("cursor overflow at expect_command"),
        }
    }
    let end = tokenizer.cursor;

    let slice = &msg_content[end..];
    let flag_key = &msg_content[start..end];

    if slice.trim_start().starts_with(&flag_key_prefix) || slice.trim().is_empty() {
        // a switch
        output.switches.insert(flag_key);
    } else {
        // expect a value qwq
        expect_flag_value(msg_content, flag_key, tokenizer, output)?;
    }
    Ok(())
}

fn expect_flag_value<'a>(
    msg_content: &'a str,
    flag_key: &'a str,
    tokenizer: &mut Tokenizer,
    output: &mut ParsedArgs<'a>,
) -> Result<(), Box<ParsedArgsError>> {
    expect_whitespace(tokenizer)?;
    let slice = &msg_content[tokenizer.cursor..];

    let prefixes = ["\"", "`", "```"];
    if let Some(prefix) = prefixes.into_iter().find(|v| slice.starts_with(v)) {
        let long_string_arg = expect_string_arg(msg_content, prefix, tokenizer)?;
        output.flags.insert(flag_key, long_string_arg);
        return Ok(());
    }

    let start = tokenizer.cursor;
    while !tokenizer.is_done(0) {
        match tokenizer.chars.get(tokenizer.cursor) {
            Some(c) if c.is_whitespace() => break,
            Some(_) => tokenizer.cursor += 1,
            None => unreachable!("cursor overflow at expect_command"),
        }
    }
    let end = tokenizer.cursor;

    let arg = &msg_content[start..end];
    output.flags.insert(flag_key, arg);
    Ok(())
}

fn expect_positional_arg<'a>(
    msg_content: &'a str,
    tokenizer: &mut Tokenizer,
    output: &mut ParsedArgs<'a>,
) -> Result<(), Box<ParsedArgsError>> {
    let slice = &msg_content[tokenizer.cursor..];

    let prefixes = ["\"", "`", "```"];
    if let Some(prefix) = prefixes.into_iter().find(|v| slice.starts_with(v)) {
        let long_string_arg = expect_string_arg(msg_content, prefix, tokenizer)?;
        output.positional.push(long_string_arg);
        return Ok(());
    }

    let start = tokenizer.cursor;
    while !tokenizer.is_done(0) {
        match tokenizer.chars.get(tokenizer.cursor) {
            Some(c) if c.is_whitespace() => break,
            Some(_) => tokenizer.cursor += 1,
            None => unreachable!("cursor overflow at expect_command"),
        }
    }
    let end = tokenizer.cursor;

    let arg = &msg_content[start..end];
    output.positional.push(arg);
    Ok(())
}

fn expect_string_arg<'a>(
    msg_content: &'a str,
    prefix: &str,
    tokenizer: &mut Tokenizer,
) -> Result<&'a str, Box<ParsedArgsError>> {
    use ParsedArgsError::ExpectedClosingDelimiter;

    tokenizer.cursor += prefix.len();
    let start = tokenizer.cursor;
    while !tokenizer.is_done(0) {
        match &msg_content[tokenizer.cursor..] {
            s if s.starts_with(prefix) => break,
            s if s.trim().is_empty() => return Err(Box::new(ExpectedClosingDelimiter)),
            _ => tokenizer.cursor += 1,
        }
    }
    let end = tokenizer.cursor;
    tokenizer.cursor += prefix.len();

    Ok(&msg_content[start..end])
}
