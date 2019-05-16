pub mod cond;

use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub enum ExpressionError {
    /// In case of division by zero.
    DivByZero,
    /// In case an invalid variable in references from the expression.
    InvalidVariable,
    /// In case of an invalid operation.
    InvalidOperation,
}

pub trait Expression: Debug + Clone + PartialEq {
    type Element: Debug + Copy + Clone + PartialEq + PartialOrd;
    /// Evaluates the expression with the given variables bound.
    fn evaluate(&self, variables: &[Self::Element]) -> Result<Self::Element, ExpressionError>;
}

pub trait Condition: Debug + Clone + PartialEq {
    type Expr: Expression;
    /// Evaluate the condition with the given variables bound.
    fn evaluate(
        &self,
        variables: &[<Self::Expr as Expression>::Element],
    ) -> Result<bool, ExpressionError>;
}
