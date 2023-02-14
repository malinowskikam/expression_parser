use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::enums::ExpressionType;
use crate::errors::AttachImpossible;

static EXP_SETTINGS: ExpressionSettings = ExpressionSettings::get_default();

pub struct ExpressionArgs {
    pub functions: HashMap<String, Box<dyn Fn(f64) -> f64>>,
    pub variables: HashMap<String, f64>,
}

pub struct ExpressionSettings {
    pub f64_delta: f64,
}

impl Default for ExpressionSettings {
    fn default() -> Self {
        ExpressionSettings::get_default()
    }
}

impl ExpressionSettings {
    const fn get_default() -> Self {
        ExpressionSettings {
            f64_delta: 1e-10,
        }
    }
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
pub struct Addition {
    pub left: Box<dyn Expression>,
    pub right: Option<Box<dyn Expression>>,
}

#[derive(Clone)]
pub struct Subtraction {
    pub left: Box<dyn Expression>,
    pub right: Option<Box<dyn Expression>>,
}

#[derive(Clone)]
pub struct Multiplication {
    pub left: Box<dyn Expression>,
    pub right: Option<Box<dyn Expression>>,
}

#[derive(Clone)]
pub struct Division {
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
    fn can_evaluate(&self, _args: &ExpressionArgs) -> bool { true }
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
            ExpressionType::Addition => Ok(Box::from(Addition {left: Box::from(self.clone()), right: None})),
            ExpressionType::Subtraction => Ok(Box::from(Subtraction {left: Box::from(self.clone()), right: None})),
            ExpressionType::Multiplication => Ok(Box::from(Multiplication {left: Box::from(self.clone()), right: None})),
            ExpressionType::Division => Ok(Box::from(Division {left: Box::from(self.clone()), right: None})),
        }
    }
}

impl Expression for Addition {
    fn evaluate(&self, args: &ExpressionArgs) -> f64 {
        match &self.right {
            None => panic!("Attempt to evaluate sum with missing right side"),
            Some(exp_box) => {
                self.left.evaluate(args) +exp_box.as_ref().evaluate(args)
            },
        }
    }
    fn to_string(&self) -> String {
        format!("{} + {}", self.left, self.right.as_ref().unwrap().clone_box())
    }
    fn get_exp_type(&self) -> ExpressionType { ExpressionType::Addition }
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>> {
        match self.right {
            None => { Ok(Box::from(Addition {left: self.left.clone_box(), right: Some(exp.clone_box())})) }
            Some(_) => Err(Box::from(AttachImpossible { target_type: self.get_exp_type(), attach_type: exp.get_exp_type() })),
        }
    }
}

impl Expression for Subtraction {
    fn evaluate(&self, args: &ExpressionArgs) -> f64 {
        match &self.right {
            None => panic!("Attempt to evaluate subtraction with missing right side"),
            Some(exp_box) => {
                self.left.evaluate(args) - exp_box.as_ref().evaluate(args)
            },
        }
    }
    fn to_string(&self) -> String {
        format!("{} - {}", self.left, self.right.as_ref().unwrap().clone_box())
    }
    fn get_exp_type(&self) -> ExpressionType { ExpressionType::Subtraction }
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>> {
        match self.right {
            None => { Ok(Box::from(Subtraction {left: self.left.clone_box(), right: Some(exp.clone_box())})) }
            Some(_) => Err(Box::from(AttachImpossible { target_type: self.get_exp_type(), attach_type: exp.get_exp_type() })),
        }
    }
}

impl Expression for Multiplication {
    fn evaluate(&self, args: &ExpressionArgs) -> f64 {
        match &self.right {
            None => panic!("Attempt to evaluate multiplication with missing right side"),
            Some(exp_box) => {
                self.left.evaluate(args) * exp_box.as_ref().evaluate(args)
            },
        }
    }
    fn to_string(&self) -> String {
        format!("{} * {}", self.left, self.right.as_ref().unwrap().clone_box())
    }
    fn get_exp_type(&self) -> ExpressionType { ExpressionType::Multiplication }
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>> {
        match self.right {
            None => { Ok(Box::from(Multiplication {left: self.left.clone_box(), right: Some(exp.clone_box())})) }
            Some(_) => Err(Box::from(AttachImpossible { target_type: self.get_exp_type(), attach_type: exp.get_exp_type() })),
        }
    }
}

impl Expression for Division {
    fn can_evaluate(&self, args: &ExpressionArgs) -> bool {
        match &self.right {
            None => panic!("Attempt to evaluate division with missing right side"),
            Some(exp_box) => {
                exp_box.as_ref().evaluate(args).abs() > EXP_SETTINGS.f64_delta
            },
        }
    }
    fn evaluate(&self, args: &ExpressionArgs) -> f64 {
        match &self.right {
            None => panic!("Attempt to evaluate division with missing right side"),
            Some(exp_box) => {
                self.left.evaluate(args) / exp_box.as_ref().evaluate(args)
            },
        }
    }
    fn to_string(&self) -> String {
        format!("{} / {}", self.left, self.right.as_ref().unwrap().clone_box())
    }
    fn get_exp_type(&self) -> ExpressionType { ExpressionType::Division }
    fn attach_after(&self, exp: &Box<dyn Expression>) -> Result<Box<dyn Expression>, Box<dyn Error>> {
        match self.right {
            None => { Ok(Box::from(Division {left: self.left.clone_box(), right: Some(exp.clone_box())})) }
            Some(_) => Err(Box::from(AttachImpossible { target_type: self.get_exp_type(), attach_type: exp.get_exp_type() })),
        }
    }
}



