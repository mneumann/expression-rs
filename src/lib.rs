#![feature(zero_one, box_syntax)]

extern crate asexp;

pub mod num_expr;

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

pub trait Expression<T>: Debug+Clone+PartialEq
where T: Debug+Copy+Clone+PartialEq+PartialOrd
{
    /// Evaluates the expression with the given variables bound.
    fn evaluate<V>(&self, variables: &[V]) -> Result<T, ExpressionError> where V: Expression<T>;
}
