#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferState {
    Empty,
    Number,
    Name,
    Bracket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharType {
    Number,
    Letter,
    Operator,
    Whitespace,
    Bracket,
    Point,
    Unknown,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug)]
pub enum ExpressionType {
    ScalarValue,
    Addition,
    Subtraction,
    Multiplication,
    Division
}

impl CharType {
    pub(crate) fn parse_char_type(character: char) -> CharType {
        match character {
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
    pub(crate) fn parse_operator_type(character: char) -> OperatorType {
        match &character {
            '+' => OperatorType::Add,
            '-' => OperatorType::Subtract,
            '*' => OperatorType::Multiply,
            '/' => OperatorType::Divide,
            '^' => OperatorType::Power,
            _ => panic!("Unknown operator type. should never happen")
        }
    }
}