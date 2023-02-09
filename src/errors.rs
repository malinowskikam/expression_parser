use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

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


