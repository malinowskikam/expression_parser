use std::error::Error;
use std::fmt::{Debug, Formatter};
use crate::enums::{BufferState, CharType, OperatorType};
use crate::errors::{EmptyBuffer, InvalidCharacter, ParsingError};
use crate::expression::{Expression, ScalarValue, Subtraction, Addition, Multiplication, Division};

struct ParserContext {
    buffer: String,
    state: BufferState,
    expression: Option<Box<dyn Expression>>,
}

impl Debug for ParserContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let exp_repr = if let Some(exp_box) = &self.expression { exp_box.as_ref().to_string() } else { String::from("") };
        write!(f, "{{ buffer = {:?}, state = {:?}, exp = {:?} }}", self.buffer, self.state, exp_repr)
    }
}

impl ParserContext {
    fn attach_exp(&mut self, exp: &Box<dyn Expression>) -> Result<(), Box<dyn Error>> {
        match &mut self.expression {
            Some(exp_box) => self.expression = Some(exp_box.as_ref().attach_after(exp)?),
            None => self.expression = Some(exp.clone_box()),
        }
        Ok(())
    }
}

fn apply_buffer(context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    let parsed_exp = parse_buffer(context)?;
    context.attach_exp(&parsed_exp)?;
    context.buffer = String::new();
    context.state = BufferState::Empty;
    Ok(())
}

fn parse_buffer(context: &mut ParserContext) -> Result<Box<dyn Expression>, Box<dyn Error>> {
    match context.state {
        BufferState::Empty => return Err(Box::from(ParsingError { message: String::from("Empty buffer!") })),
        BufferState::Number => {
            let value = context.buffer.parse::<f64>()?;
            Ok(Box::from(ScalarValue { value }) as Box<dyn Expression>)
        }
        _ => todo!(),
    }
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
            match context.expression {
                Some(ref exp) => {
                    match operator_type {
                        OperatorType::Add => context.expression = Some(Box::from(Addition { left: exp.clone_box(), right: None })),
                        OperatorType::Subtract => context.expression = Some(Box::from(Subtraction { left: exp.clone_box(), right: None })),
                        OperatorType::Multiply => context.expression = Some(Box::from(Multiplication { left: exp.clone_box(), right: None })),
                        OperatorType::Divide => context.expression = Some(Box::from(Division { left: exp.clone_box(), right: None })),
                        OperatorType::Power => todo!(),
                    }
                }
                None => {
                    match operator_type {
                        OperatorType::Subtract => {
                            context.state = BufferState::Number;
                            context.buffer.push(character)
                        },
                        _ => return Err(Box::from(InvalidCharacter { character, index, message: String::from("Operator at the start of a block") })),
                    }
                },
            }
        }
        CharType::Whitespace => (),
        CharType::Bracket => todo!(),
        CharType::Point => return Err(Box::from(InvalidCharacter { character, index, message: String::from("Point at the start of a block") })),
        CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: String::from("Unknown symbol") })),
    };
    Ok(())
}

fn parse_number(character: char, char_type: CharType, index: usize, context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    match char_type {
        CharType::Number => context.buffer.push(character),
        CharType::Letter => return Err(Box::from(InvalidCharacter { character, index, message: String::from("Letter inside number") })), //todo implicit multiplication
        CharType::Operator => {
            apply_buffer(context)?;
            context.state = BufferState::Empty;
            parse_empty(character, char_type, index, context)?;
        },
        CharType::Whitespace => {
            apply_buffer(context)?;
            context.state = BufferState::Empty;
        },
        CharType::Bracket => todo!(),
        CharType::Point => context.buffer.push(character),
        CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: String::from("Unknown symbol") })),
    }
    Ok(())
}

fn parse_name(character: char, char_type: CharType, index: usize, context: &mut ParserContext) -> Result<(), Box<dyn Error>> {
    match char_type {
        CharType::Number => context.buffer.push(character),
        CharType::Letter => context.buffer.push(character),
        CharType::Operator => {
            apply_buffer(context)?;
            context.state = BufferState::Empty;
            parse_empty(character, char_type, index, context)?
        },
        CharType::Whitespace => {
            apply_buffer(context)?;
            context.state = BufferState::Empty;
        },
        CharType::Bracket => todo!(),
        CharType::Point => context.buffer.push(character),
        CharType::Unknown => return Err(Box::from(InvalidCharacter { character, index, message: String::from("Unknown symbol") })),
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
            BufferState::Number => parse_number(character, char_type, index, &mut context)?,
            BufferState::Name => parse_name(character, char_type, index, &mut context)?,
            BufferState::Bracket => todo!(),
        };
    }

    if !context.buffer.is_empty() {
        apply_buffer(&mut context)?;
    }

    match context.expression {
        None => Err(Box::from(EmptyBuffer)),
        Some(expression_box) => Ok(expression_box),
    }
}