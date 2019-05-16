use super::{Condition, Expression, ExpressionError};
use asexp::Sexp;
use std::fmt::Debug;

/// A boolean condition evaluates to either `true` or `false`.
#[derive(Debug, Clone, PartialEq)]
pub enum Cond<E: Expression> {
    True,
    False,
    Not(Box<Cond<E>>),
    And(Box<Cond<E>>, Box<Cond<E>>),
    Or(Box<Cond<E>>, Box<Cond<E>>),

    /// If two expressions are equal
    Equal(Box<E>, Box<E>),

    Less(Box<E>, Box<E>),
    Greater(Box<E>, Box<E>),

    LessEqual(Box<E>, Box<E>),
    GreaterEqual(Box<E>, Box<E>),
}

impl<E: Expression> Condition for Cond<E> {
    type Expr = E;
    fn evaluate(
        &self,
        variables: &[<Self::Expr as Expression>::Element],
    ) -> Result<bool, ExpressionError> {
        Ok(match *self {
            Cond::True => true,
            Cond::False => false,
            Cond::Not(ref c) => !c.evaluate(variables)?,
            Cond::And(ref c1, ref c2) => c1.evaluate(variables)? && c2.evaluate(variables)?,
            Cond::Or(ref c1, ref c2) => c1.evaluate(variables)? || c2.evaluate(variables)?,
            Cond::Equal(ref e1, ref e2) => e1.evaluate(variables)? == e2.evaluate(variables)?,
            Cond::Less(ref e1, ref e2) => e1.evaluate(variables)? < e2.evaluate(variables)?,
            Cond::Greater(ref e1, ref e2) => e1.evaluate(variables)? > e2.evaluate(variables)?,
            Cond::LessEqual(ref e1, ref e2) => e1.evaluate(variables)? <= e2.evaluate(variables)?,
            Cond::GreaterEqual(ref e1, ref e2) => {
                e1.evaluate(variables)? >= e2.evaluate(variables)?
            }
        })
    }
}

impl<'a, E, T> Into<Sexp> for &'a Cond<E>
where
    E: Expression<Element = T>,
    &'a E: Into<Sexp>,
    T: Debug + Copy + Clone + PartialEq + PartialOrd,
{
    fn into(self) -> Sexp {
        match self {
            &Cond::True => Sexp::from("true"),
            &Cond::False => Sexp::from("false"),
            &Cond::Not(ref a) => Sexp::from(("not", Into::<Sexp>::into(a.as_ref()))),
            &Cond::And(ref a, ref b) => Sexp::from((
                "and",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
            &Cond::Or(ref a, ref b) => Sexp::from((
                "or",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
            &Cond::Equal(ref a, ref b) => Sexp::from((
                "==",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
            &Cond::Less(ref a, ref b) => Sexp::from((
                "<",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
            &Cond::Greater(ref a, ref b) => Sexp::from((
                ">",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
            &Cond::LessEqual(ref a, ref b) => Sexp::from((
                "<=",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
            &Cond::GreaterEqual(ref a, ref b) => Sexp::from((
                ">=",
                Into::<Sexp>::into(a.as_ref()),
                Into::<Sexp>::into(b.as_ref()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cond::Cond, Condition, Expression, ExpressionError};

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct ConstNum(f32);

    impl Expression for ConstNum {
        type Element = f32;
        fn evaluate(&self, _variables: &[Self::Element]) -> Result<Self::Element, ExpressionError> {
            Ok(self.0)
        }
    }

    #[test]
    fn test_condition() {
        let no_vars: &[f32] = &[];

        let cond = Cond::Greater(Box::new(ConstNum(0.1)), Box::new(ConstNum(0.2)));
        assert_eq!(Ok(false), cond.evaluate(no_vars));

        let cond = Cond::Not(Box::new(Cond::Greater(
            Box::new(ConstNum(0.1)),
            Box::new(ConstNum(0.2)),
        )));
        assert_eq!(Ok(true), cond.evaluate(no_vars));
    }
}
