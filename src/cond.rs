use super::{Expression, ExpressionError, ElementType, Condition};
use asexp::Sexp;

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
    fn evaluate(&self, variables: &[Self::Expr]) -> Result<bool, ExpressionError> {
        Ok(match *self {
            Cond::True => true,
            Cond::False => false,
            Cond::Not(ref c) => !try!(c.evaluate(variables)),
            Cond::And(ref c1, ref c2) => {
                try!(c1.evaluate(variables)) && try!(c2.evaluate(variables))
            }
            Cond::Or(ref c1, ref c2) => {
                try!(c1.evaluate(variables)) || try!(c2.evaluate(variables))
            }
            Cond::Equal(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) == try!(e2.evaluate(variables))
            }
            Cond::Less(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) < try!(e2.evaluate(variables))
            }
            Cond::Greater(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) > try!(e2.evaluate(variables))
            }
            Cond::LessEqual(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) <= try!(e2.evaluate(variables))
            }
            Cond::GreaterEqual(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) >= try!(e2.evaluate(variables))
            }
        })
    }
}

impl<'a, E, T> Into<Sexp> for &'a Cond<E>
    where E: Expression<Element = T>,
          &'a E: Into<Sexp>,
          T: ElementType + Into<Sexp>
{
    fn into(self) -> Sexp {
        match self {
            &Cond::True => Sexp::from("true"),
            &Cond::False => Sexp::from("false"),
            &Cond::Not(ref a) => Sexp::from(("not", Into::<Sexp>::into(a.as_ref()))),
            &Cond::And(ref a, ref b) => {
                Sexp::from(("and",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Cond::Or(ref a, ref b) => {
                Sexp::from(("or",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Cond::Equal(ref a, ref b) => {
                Sexp::from(("==",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Cond::Less(ref a, ref b) => {
                Sexp::from(("<",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Cond::Greater(ref a, ref b) => {
                Sexp::from((">",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Cond::LessEqual(ref a, ref b) => {
                Sexp::from(("<=",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Cond::GreaterEqual(ref a, ref b) => {
                Sexp::from((">=",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
        }
    }
}

#[test]
fn test_condition() {
    use super::num_expr::NumExpr;
    let cond = Cond::Greater(box NumExpr::Var(0), box NumExpr::Const(0.0));

    fn fun(a: f32) -> bool {
        a > 0.0
    }

    fn check(cond: &Cond<NumExpr<f32>>, a: f32) {
        assert_eq!(Ok(fun(a)), cond.evaluate(&[NumExpr::Const(a)]))
    }

    check(&cond, 123.0);
    check(&cond, 0.0);
    check(&cond, -1.4);

    let no_vars: &[NumExpr<f32>] = &[];
    assert_eq!(Ok(true), Cond::True.evaluate(no_vars));
}