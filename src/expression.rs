use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::errors::AttachImpossible;

#[derive(Debug)]
pub enum ExpressionType {
    ScalarValue,
    Sum
}

pub struct ExpressionArgs {
    pub functions: HashMap<String, Box<dyn Fn(f64) -> f64>>,
    pub variables: HashMap<String, f64>,
}

pub trait ExpressionClone {
    fn clone_box(&self) -> Box<dyn Expression>;
}

impl<T: 'static + Expression + Clone> ExpressionClone for T {
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct ScalarValue {
    pub value: f64,
}

#[derive(Clone)]
pub struct Sum {
    pub left: Box<dyn Expression>,
    pub right: Option<Box<dyn Expression>>,
}

impl ExpressionArgs {
    pub fn empty() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Box<dyn Expression> {
        self.clone_box()
    }
}

pub trait Expression: ExpressionClone {
    fn can_evaluate(&self, args: &ExpressionArgs) -> bool { true }
    fn evaluate(&self, args: &ExpressionArgs) -> f64;
    fn to_string(&self) -> String;
    fn get_exp_type(&self) -> ExpressionType;
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>>;
}

impl Display for dyn Expression { fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.to_string()) } }

impl Expression for ScalarValue {
    fn evaluate(&self, _args: &ExpressionArgs) -> f64 { self.value }
    fn to_string(&self) -> String { format!("{}", self.value) }
    fn get_exp_type(&self) -> ExpressionType { ExpressionType::ScalarValue }
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>> {
        match exp.get_exp_type() {
            ExpressionType::ScalarValue => Err(Box::from(AttachImpossible { target_type: self.get_exp_type(), attach_type: exp.get_exp_type() })),
            ExpressionType::Sum => Ok(Box::from(Sum {left: Box::from(ScalarValue{value: self.value}), right: None}))
        }
    }
}

impl Expression for Sum {
    fn evaluate(&self, args: &ExpressionArgs) -> f64 {
        match &self.right {
            None => panic!("Attempt to evaluate sum with missing right side"),
            Some(exp_box) => {
                self.left.evaluate(args) +exp_box.as_ref().evaluate(args)
            },
        }
    }
    fn to_string(&self) -> String { format!("{} + {}", self.left, self.right.as_ref().unwrap().clone_box()) }
    fn get_exp_type(&self) -> ExpressionType { ExpressionType::Sum }
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>> {
        match self.right {
            None => { Ok(Box::from(Sum {left: self.left.clone_box(), right: Some(self.right.as_ref().unwrap().clone_box())})) }
            Some(_) => Err(Box::from(AttachImpossible { target_type: self.get_exp_type(), attach_type: exp.get_exp_type() })),
        }
    }
}



