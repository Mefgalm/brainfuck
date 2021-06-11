#[derive(Debug)]
pub enum Expression {
    IncPointer,
    DecPointer,
    IncData,
    DecData,
    Output,
    Accept,
    Loop(Vec<Expression>)
}

#[derive(Debug)]
pub enum ParseError {
    InvalidToken
}

struct StringReader {
    chars: Vec<char>,
    position: usize
}

impl StringReader {
    fn new(input: &str) -> StringReader {
        StringReader {
            chars: input.bytes().map(|b| b as char).collect::<Vec<char>>(),
            position: 0
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.position >= self.chars.len() {
            None
        } else {
            let c = self.chars[self.position];
            self.position += 1;                       
            Some(c)
        }
    }
}

fn parse_expression(c: char, sr: &mut StringReader) -> Option<Result<Expression, ParseError>> {
    match c {
        '>' => Some(Ok(Expression::IncPointer)),
        '<' => Some(Ok(Expression::DecPointer)),
        '+' => Some(Ok(Expression::IncData)),
        '-' => Some(Ok(Expression::DecData)),
        '.' => Some(Ok(Expression::Output)),
        ',' => Some(Ok(Expression::Accept)),
        '[' => Some(parse_loop(sr)),
        _ => None 
    }
}

fn parse_loop(sr: &mut StringReader) -> Result<Expression, ParseError> {
    let mut expressions = vec![];
    while let Some(c) = sr.next() {
        if c == ']' {
            return Ok(Expression::Loop(expressions));
        }
        if let Some(expression_result) = parse_expression(c, sr) {
            expressions.push(expression_result?);
        }
    }
    Err(ParseError::InvalidToken)
}

pub fn parse(input: &str) -> Result<Vec<Expression>, ParseError> {
    let mut sr = StringReader::new(input);
    let mut expressions = vec![];
    while let Some(c) = sr.next() {
        if let Some(expression_result) = parse_expression(c, &mut sr) {
            expressions.push(expression_result?);
        }
    }
    Ok(expressions)
}

