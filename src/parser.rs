use std::error::Error;
use crate::enums::{BufferState, CharType, OperatorType};
use crate::errors::{EmptyBuffer, InvalidCharacter, ParsingError};
use crate::expression::{Expression, ScalarValue, Sum};
struct ParserContext {
    buffer: String,
    state: BufferState,
    expression: Option<Box<dyn Expression>>,
}

impl ParserContext {
    fn attach_exp(&mut self, exp: &Box<dyn Expression>) -> Result<(), Box<dyn Error>> {
        match &mut self.expression {
            None => self.expression = Some(exp.clone_box()),
            Some(exp_box) => { self.expression = Some(exp_box.as_ref().attach_after(exp)?) }
        }
        Ok(())
    }
}

fn parse_buffer(context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    let result = match context.state {
        BufferState::Empty => return Err(Box::from(ParsingError { message: "Empty buffer!" })),
        BufferState::Number => {
            let result = context.buffer.parse::<f64>();
            match result {
                Ok(v) => Ok(Box::from(ScalarValue { value: v }) as Box<dyn Expression>),
                Err(e) => Err(format!("Error while parsing number: {}", e.to_string()))
            }
        }
        _ => todo!(),
    };
    context.attach_exp(&result?)
}

fn parse_empty(character: char, char_type: CharType, index: usize, context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    match char_type {
        CharType::Number => {
            context.state = BufferState::Number;
            context.buffer.push(character)
        }
        CharType::Letter => {
            context.state = BufferState::Name;
            context.buffer.push(character)
        }
        CharType::Operator => {
            let operator_type = OperatorType::parse_operator_type(character);
            match operator_type {
                OperatorType::Subtract => {
                    context.state = BufferState::Number;
                    context.buffer.push(character)
                }
                _ => return Err(Box::from(InvalidCharacter { character, index, message: "Operator at the start of a block" })),
            }
        }
        CharType::Whitespace => (),
        CharType::Bracket => todo!(),
        CharType::Point => return Err(Box::from(InvalidCharacter { character, index, message: "Point at the start of a block" })),
        CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: "Unknown symbol" })),
    };
    Ok(())
}

fn parse_exp_terminated(character: char, char_type: CharType, index: usize, context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    match char_type {
        CharType::Operator => {
            let operator_type = OperatorType::parse_operator_type(character);
            match operator_type {
                OperatorType::Add => match context.expression {
                    Some(ref exp) => context.expression = Some(Box::from(Sum { left: exp.clone_box(), right: None })),
                    None => panic!("Operator after exp terminated, but expression is None. Should be unreachable state"),
                },
                OperatorType::Subtract => todo!(),
                OperatorType::Multiply => todo!(),
                OperatorType::Divide => todo!(),
                OperatorType::Power => todo!(),
                OperatorType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: "Operator at the start of a block" })),
            }
        }
        CharType::Whitespace => (),
        CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: "Unknown symbol" })),
        _ => return Err(Box::from(InvalidCharacter { character, index, message: "Expected operator after previous expression" })),
    };
    Ok(())
}

fn parse_number(character: char, char_type: CharType, index: usize, context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    match char_type {
        CharType::Number => context.buffer.push(character),
        CharType::Letter => return Err(Box::from(InvalidCharacter { character, index, message: "Letter inside number" })), //todo implicit multiplication
        CharType::Operator => {
            parse_buffer(context)?;
            context.state = BufferState::ExpTerminated;
            let operator_type = OperatorType::parse_operator_type(character);
            match operator_type {
                OperatorType::Add => match context.expression {
                    Some(ref exp) => context.expression = Some(Box::from(Sum { left: exp.clone_box(), right: None })),
                    None => panic!("Operator after exp terminated, but expression is None. Should be unreachable state"),
                },
                OperatorType::Subtract => todo!(),
                OperatorType::Multiply => todo!(),
                OperatorType::Divide => todo!(),
                OperatorType::Power => todo!(),
                OperatorType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: "Operator at the start of a block" })),
            }
        }
        CharType::Whitespace => {
            parse_buffer(context)?;
            context.state = BufferState::ExpTerminated;
        }
        CharType::Bracket => todo!(),
        CharType::Point => context.buffer.push(character),
        CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: "Unknown symbol" })),
    }
    Ok(())
}

fn parse_name(character: char, char_type: CharType, index: usize, context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    match char_type {
                CharType::Number => context.buffer.push(character),
                CharType::Letter => context.buffer.push(character),
                CharType::Operator => todo!(),
                CharType::Whitespace => todo!(),
                CharType::Bracket => todo!(),
                CharType::Point => todo!(),
                CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: "Unknown symbol" })),
            }
    Ok(())
}

pub fn parse_string(string_to_parse: String) -> Result<Box<dyn Expression>, Box<dyn Error>> {
    let mut context = ParserContext {
        buffer: String::new(),
        state: BufferState::Empty,
        expression: None,
    };

    for (index, character) in string_to_parse.chars().enumerate() {
        let char_type = CharType::parse_char_type(character);

        match context.state {
            BufferState::Empty => parse_empty(character, char_type, index, &mut context)?,
            BufferState::ExpTerminated => parse_exp_terminated(character, char_type, index, &mut context)?,
            BufferState::Number => parse_number(character, char_type, index, &mut context)?,
            BufferState::Name => parse_name(character, char_type, index, &mut context)?,
            BufferState::Bracket => todo!(),
        };
    }

    if !context.buffer.is_empty() {
        parse_buffer(&mut context)?;
    }

    match context.expression {
        None => Err(Box::from(EmptyBuffer)),
        Some(expression_box) => Ok(expression_box),
    }
}