use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct ExpressionArgs {
    pub functions: HashMap<String, Box<dyn Fn(f64) -> f64>>,
    pub variables: HashMap<String, f64>,
}

pub trait Expression {
    fn evaluate(&self, args: &ExpressionArgs) -> f64;
    fn to_string(&self) -> String;
}

pub struct ScalarValue {
    pub value: f64,
}

pub struct Sum {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl ExpressionArgs {
    pub fn empty() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

impl Display for dyn Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Expression for ScalarValue {
    fn evaluate(&self, _args: &ExpressionArgs) -> f64 { self.value }
    fn to_string(&self) -> String { format!("{}", self.value) }
}

impl Expression for Sum {
    fn evaluate(&self, args: &ExpressionArgs) -> f64 { self.left.evaluate(args) + self.right.evaluate(args) }
    fn to_string(&self) -> String { format!("{} + {}", self.left, self.right) }
}


