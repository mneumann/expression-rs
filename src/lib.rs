#![feature(box_syntax)]

extern crate asexp;

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

pub trait ElementType: Debug + Copy + Clone + PartialEq + PartialOrd {}

impl ElementType for f32 {}
impl ElementType for f64 {}
impl ElementType for u32 {}
impl ElementType for u64 {}
impl ElementType for i32 {}
impl ElementType for i64 {}
impl ElementType for usize {}
impl ElementType for isize {}

pub trait Expression: Debug + Clone + PartialEq
{
    type Element: ElementType;
    /// Evaluates the expression with the given variables bound.
    fn evaluate(&self, variables: &[Self::Element]) -> Result<Self::Element, ExpressionError>;
}

pub trait Condition: Debug + Clone + PartialEq
{
    type Expr: Expression;
    /// Evaluate the condition with the given variables bound.
    fn evaluate(&self,
                variables: &[<Self::Expr as Expression>::Element])
                -> Result<bool, ExpressionError>;
}
