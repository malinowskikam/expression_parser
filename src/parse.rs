use std::error::Error;
use crate::errors::{EmptyBuffer, InvalidCharacter};
use crate::expression::{Expression, ScalarValue};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BufferState {
    Empty,
    Number,
    Name,
    Bracket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CharType {
    Number,
    Letter,
    Operator,
    Whitespace,
    Bracket,
    Point,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Unknown,
}

impl CharType {
    fn parse_char_type(char_arg: char) -> CharType {
        match char_arg {
            char_arg if char_arg.is_numeric() => CharType::Number,
            char_arg if "+-*/^".contains(char_arg) => CharType::Operator,
            char_arg if char_arg.is_alphabetic() => CharType::Letter,
            '.' => CharType::Point,
            char_arg if char_arg.is_whitespace() => CharType::Whitespace,
            char_arg if "(){}[]".contains(char_arg) => CharType::Bracket,
            _ => CharType::Unknown
        }
    }
}

impl OperatorType {
    fn parse_operator_type(char_arg: char) -> OperatorType {
        match char_arg {
            '+' => OperatorType::Add,
            '-' => OperatorType::Subtract,
            '*' => OperatorType::Multiply,
            '/' => OperatorType::Divide,
            '^' => OperatorType::Power,
            _ => OperatorType::Unknown
        }
    }
}

fn parse_buffer(parser_buffer: String, buffer_state: BufferState) -> Result<Box<dyn Expression>, String> {
    match buffer_state {
        BufferState::Empty => return Err("Attempt to parse buffer in empty state".to_string()),
        BufferState::Number => {
            let result = parser_buffer.parse::<f64>();
            match result {
                Ok(v) => Ok(Box::from(ScalarValue { value: v })),
                Err(e) => Err(format!("Error while parsing number: {}", e.to_string()))
            }
        }
        _ => todo!(),
    }
}

pub fn parse_string(string_to_parse: String) -> Result<Box<dyn Expression>, Box<dyn Error>> {
    let mut parser_buffer = String::new();
    let mut buffer_state = BufferState::Empty;
    let mut current_expression : Option<Box<dyn Expression>> = None;

    for (index, character) in string_to_parse.chars().enumerate() {
        let char_type = CharType::parse_char_type(character);

        match buffer_state {
            BufferState::Empty => match char_type {
                CharType::Number => {
                    buffer_state = BufferState::Number;
                    parser_buffer.push(character)
                }
                CharType::Letter => {
                    buffer_state = BufferState::Name;
                    parser_buffer.push(character)
                }
                CharType::Operator => return Err(Box::from(InvalidCharacter {character, index, message: "Operator at the start of a block"})),
                CharType::Whitespace => todo!(),
                CharType::Bracket => todo!(),
                CharType::Point => return Err(Box::from(InvalidCharacter {character, index, message: "Point at the start of a block"})),
                CharType::Unknown => return Err(Box::from(InvalidCharacter {character, index, message: "Unknown symbol"})),
            },
            BufferState::Number => match char_type {
                CharType::Number => parser_buffer.push(character),
                CharType::Letter => return Err(Box::from(InvalidCharacter {character, index, message: "Letter inside number"})), //todo implicit multiplication
                CharType::Operator => todo!(),
                CharType::Whitespace => todo!(),
                CharType::Bracket => todo!(),
                CharType::Point => parser_buffer.push(character),
                CharType::Unknown => return Err(Box::from(InvalidCharacter {character, index, message: "Unknown symbol"})),
            },
            BufferState::Name => match char_type {
                CharType::Number => parser_buffer.push(character),
                CharType::Letter => parser_buffer.push(character),
                CharType::Operator => todo!(),
                CharType::Whitespace => todo!(),
                CharType::Bracket => todo!(),
                CharType::Point => todo!(),
                CharType::Unknown => return Err(Box::from(InvalidCharacter {character, index, message: "Unknown symbol"})),
            },
            BufferState::Bracket => (),
        };
    }

    if !parser_buffer.is_empty() {
        let result = parse_buffer(parser_buffer, buffer_state)?;
        match current_expression {
            None => current_expression = Some(result),
            Some(expression_box) => todo!(),
        }
    }

    match current_expression {
        None => Err(Box::from(EmptyBuffer)),
        Some(expression_box) => Ok(expression_box),
    }
}