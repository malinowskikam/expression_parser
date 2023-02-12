use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::enums::ExpressionType;

#[derive(Debug)]
pub struct EmptyBuffer;

impl Display for EmptyBuffer { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "Empty buffer!") } }

impl Error for EmptyBuffer {}

#[derive(Debug)]
pub struct InvalidCharacter {
    pub index: usize,
    pub character: char,
    pub message: &'static str,
}

impl Display for InvalidCharacter { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "Error at char '{}' at index {} ({})", self.character, self.index, self.message) } }

impl Error for InvalidCharacter {}

#[derive(Debug)]
pub struct AttachImpossible {
    pub target_type: ExpressionType,
    pub attach_type: ExpressionType,
}

impl Display for AttachImpossible { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "Attach from {:?} to {:?} impossible", self.target_type, self.attach_type) } }

impl Error for AttachImpossible {}

#[derive(Debug)]
pub struct ParsingError {
    pub message: &'static str,
}

impl Display for ParsingError { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "Parsing buffer error ({})", self.message) } }

impl Error for ParsingError {}
