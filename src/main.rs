use expression_parser::expression::{ExpressionArgs};
use expression_parser::parser::parse_string;

fn main() {
    let expression_args = ExpressionArgs::empty();
    let input = "21 + 1";


    let result = parse_string(input.to_string());
    match result {
        Ok(expression_box) => {
            let expression = expression_box.as_ref();
            println!("EX: {}", expression);
            println!("VAL: {}", expression.evaluate(&expression_args));
        },
        Err(error_box) => {
            let error = error_box.as_ref();
            println!("Error: {}", error);
        },
    }
}


