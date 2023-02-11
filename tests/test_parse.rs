use expression_parser::errors::EmptyBuffer;
use expression_parser::expression::ExpressionArgs;
use expression_parser::parser::parse_string;

#[test]
fn test_parse_empty() {
    let result = parse_string("".to_string());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().as_ref().to_string(), "Empty buffer!")
}

#[test]
fn test_parse_unknown() {
    let result = parse_string("".to_string());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().as_ref().to_string(), "Empty buffer!")
}

#[test]
fn test_parse_value_int() {
    let result = parse_string("21".to_string());
    assert!(!result.is_err());
    assert_eq!(21.0 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty()));
}

#[test]
fn test_parse_value_float() {
    let result = parse_string("21.25".to_string());
    assert!(!result.is_err());
    assert_eq!(21.25 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty()));
}

#[test]
fn test_parse_start_dot() {
    let result = parse_string(".64".to_string());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().as_ref().to_string(), "Error at char '.' at index 0 (Point at the start of a block)")
}

#[test]
fn test_parse_start_operator() {
    let result = parse_string("*765".to_string());
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().as_ref().to_string(), "Error at char '*' at index 0 (Operator at the start of a block)")
}