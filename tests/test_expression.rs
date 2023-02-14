use std::error::Error;
use expression_parser::expression::{Expression, ExpressionArgs, ExpressionSettings};
use expression_parser::parser::parse_string;

struct Setup {
    exp_settings: ExpressionSettings,
}

impl Setup {
    fn new() -> Setup {
        Setup {
            exp_settings: ExpressionSettings::default()
        }
    }

    fn comp_with_delta(&self, val1: f64, val2: f64) -> bool {
        let delta = ((val1 + val2)/2.0).abs() * self.exp_settings.f64_delta;
        (val1 - val2).abs() < delta
    }

    fn assert_exp_result(&self, result: Result<Box<dyn Expression>, Box<dyn Error>>, expected_value: f64, expected_repr: &str) {
        assert!(!result.is_err());
        let result_exp = result.unwrap();

        let actual_repr = result_exp.as_ref().to_string();
        assert_eq!(expected_repr, actual_repr, "repr: expected = {:?}, actual = {:?}", actual_repr, actual_repr);

        let actual_value = result_exp.as_ref().evaluate(&ExpressionArgs::empty());
        assert!(self.comp_with_delta(expected_value, actual_value), "val: expected = {:?}, actual = {:?}", expected_value, actual_value);
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
    let setup = Setup::new();
    let result = parse_string("21".to_string());
    setup.assert_exp_result(result, 21.0f64, "21")
}

#[test]
fn test_parse_value_neg_int() {
    let setup = Setup::new();
    let result = parse_string("-21".to_string());
    setup.assert_exp_result(result, -21.0f64, "-21")
}

#[test]
fn test_parse_value_float() {
    let setup = Setup::new();
    let result = parse_string("21.25".to_string());
    assert!(!result.is_err());
    setup.assert_exp_result(result, 21.25f64, "21.25");
}

#[test]
fn test_parse_value_neg_float() {
    let setup = Setup::new();
    let result = parse_string("-21.25".to_string());
    assert!(!result.is_err());
    setup.assert_exp_result(result, -21.25f64, "-21.25");
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
    setup.assert_exp_result(result, 3.0f64, "1 + 2");
}

#[test]
fn test_add_neg_start() {
    let setup = Setup::new();
    let result = parse_string("-1 + 2".to_string());
    setup.assert_exp_result(result, 1.0f64, "-1 + 2");
}

#[test]
fn test_add_float() {
    let setup = Setup::new();
    let result = parse_string("1.03 + 2.07".to_string());
    setup.assert_exp_result(result, 3.1f64, "1.03 + 2.07");
}

#[test]
fn test_subtract() {
    let setup = Setup::new();
    let result = parse_string("4 - 3".to_string());
    setup.assert_exp_result(result, 1.0f64, "4 - 3");
}

#[test]
fn test_subtract_float() {
    let setup = Setup::new();
    let result = parse_string("4.17 - 2.08".to_string());
    setup.assert_exp_result(result, 2.09f64, "4.17 - 2.08");
}

#[test]
fn test_add_chain() {
    let setup = Setup::new();
    let result = parse_string("1 + 2 + 3".to_string());
    setup.assert_exp_result(result, 6.0f64, "1 + 2 + 3");
}

#[test]
fn test_add_chain_float() {
    let setup = Setup::new();
    let result = parse_string("1.03 + 2.07 + 3.05".to_string());
    setup.assert_exp_result(result, 6.15f64, "1.03 + 2.07 + 3.05");
}

#[test]
fn test_subtract_chain() {
    let setup = Setup::new();
    let result = parse_string("4 - 2 - 1".to_string());
    setup.assert_exp_result(result, 1.0f64, "4 - 2 - 1");
}

#[test]
fn test_subtract_chain_float() {
    let setup = Setup::new();
    let result = parse_string("4.17 - 2.08 - 1.03".to_string());
    setup.assert_exp_result(result, 1.06f64, "4.17 - 2.08 - 1.03");
}

#[test]
fn test_add_subtract_chain() {
    let setup = Setup::new();
    let result = parse_string("4 + 3 - 5".to_string());
    setup.assert_exp_result(result, 2.0f64, "4 + 3 - 5");
}

#[test]
fn test_subtract_add_chain() {
    let setup = Setup::new();
    let result = parse_string("4 - 3 + 5".to_string());
    setup.assert_exp_result(result, 6.0f64, "4 - 3 + 5");
}

#[test]
fn test_add_subtract_chain_float() {
    let setup = Setup::new();
    let result = parse_string("4.11 + 3.06 - 5.08".to_string());
    setup.assert_exp_result(result, 2.09f64, "4.11 + 3.06 - 5.08");
}

#[test]
fn test_subtract_add_chain_float() {
    let setup = Setup::new();
    let result = parse_string("4.04 - 3.01 + 5.09".to_string());
    setup.assert_exp_result(result, 6.12f64, "4.04 - 3.01 + 5.09");
}

#[test]
fn test_multiply() {
    let setup = Setup::new();
    let result = parse_string("4 * 3".to_string());
    setup.assert_exp_result(result, 12.0f64, "4 * 3");
}

#[test]
fn test_multiply_float() {
    let setup = Setup::new();
    let result = parse_string("4.17 * 2.08".to_string());
    setup.assert_exp_result(result, 8.6736f64, "4.17 * 2.08");
}

#[test]
fn test_divide_int() {
    let setup = Setup::new();
    let result = parse_string("4 / 2".to_string());
    setup.assert_exp_result(result, 2.0f64, "4 / 2");
}

#[test]
fn test_divide() {
    let setup = Setup::new();
    let result = parse_string("5 / 2".to_string());
    setup.assert_exp_result(result, 2.5f64, "5 / 2");
}

#[test]
fn test_divide_float() {
    let setup = Setup::new();
    let result = parse_string("4.2 / 2.2".to_string());
    setup.assert_exp_result(result, 1.90909090909f64, "4.2 / 2.2");
}

