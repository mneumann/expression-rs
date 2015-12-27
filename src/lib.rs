#![feature(zero_one, box_syntax)]

extern crate asexp;

pub mod num_expr;
pub mod cond;

use std::fmt::Debug;
use std::cmp::{PartialEq, PartialOrd};

#[derive(Debug, Eq, PartialEq)]
pub enum ExpressionError {
    /// In case of division by zero.
    DivByZero,
    /// In case an invalid variable in references from the expression.
    InvalidVariable,
    /// In case of an invalid operation.
    InvalidOperation,
}

pub trait ElementType: Debug + Copy + Clone + PartialEq + PartialOrd + Default {}

pub trait Expression: Debug + Clone + PartialEq
{
    type Element: ElementType;
    /// Evaluates the expression with the given variables bound.
    fn evaluate(&self, variables: &[Self]) -> Result<Self::Element, ExpressionError>;

    /// Constructs a constant expression. Required by lindenmayer-system.
    fn make_const(elm: Self::Element) -> Self;
}

pub trait Condition: Debug + Clone + PartialEq
{
    type Expr: Expression;
    /// Evaluate the condition with the given variables bound.
    fn evaluate(&self, variables: &[Self::Expr]) -> Result<bool, ExpressionError>;
}
