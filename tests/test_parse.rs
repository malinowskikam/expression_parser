use expression_parser::errors::EmptyBuffer;
use expression_parser::expression::ExpressionArgs;
use expression_parser::parser::parse_string;

struct Setup {
    f64_delta: f64
}

impl Setup {
    fn new() -> Setup {
        Setup {
            f64_delta: 1e-10
        }
    }

    fn comp_with_delta(&self, val1: f64, val2: f64) -> bool {
        let delta = (val1 + val2) * self.f64_delta;
        (val1 - val2).abs() < delta
    }
}

#[test]
fn test_parse_empty() {
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

#[test]
fn test_add() {
    let setup = Setup::new();
    let result = parse_string("1 + 2".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(3 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_add_float() {
    let setup = Setup::new();
    let result = parse_string("1.03 + 2.07".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(3.1 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_subtract() {
    let setup = Setup::new();
    let result = parse_string("4 - 3".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(1 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_subtract_float() {
    let setup = Setup::new();
    let result = parse_string("4.17 - 2.08".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(2.09 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_add_chain() {
    let setup = Setup::new();
    let result = parse_string("1 + 2 + 3".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(6 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_add_chain_float() {
    let setup = Setup::new();
    let result = parse_string("1.03 + 2.07 + 3.05".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(6.15 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_subtract_chain() {
    let setup = Setup::new();
    let result = parse_string("4 - 2 - 1".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(1 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_subtract_chain_float() {
    let setup = Setup::new();
    let result = parse_string("4.17 - 2.08 - 1.03".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(1.06 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_add_subtract_chain() {
    let setup = Setup::new();
    let result = parse_string("4 + 3 - 5".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(2 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_subtract_add_chain() {
    let setup = Setup::new();
    let result = parse_string("4 - 3 + 5".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(6 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_add_subtract_chain_float() {
    let setup = Setup::new();
    let result = parse_string("4.11 + 3.06 - 5.08".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(2.09 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

#[test]
fn test_subtract_add_chain_float() {
    let setup = Setup::new();
    let result = parse_string("4.04 - 3.01 + 5.09".to_string());
    assert!(!result.is_err());
    assert!(setup.comp_with_delta(6.12 as f64, result.unwrap().as_ref().evaluate(&ExpressionArgs::empty())));
}

