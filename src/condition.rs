use super::{Expression, ExpressionError, ElementType};
use asexp::Sexp;

/// A condition evaluates to either `true` or `false`.
#[derive(Debug, Clone)]
pub enum Condition<E: Expression> {
    True,
    False,
    Not(Box<Condition<E>>),
    And(Box<Condition<E>>, Box<Condition<E>>),
    Or(Box<Condition<E>>, Box<Condition<E>>),

    /// If two expressions are equal
    Equal(Box<E>, Box<E>),

    Less(Box<E>, Box<E>),
    Greater(Box<E>, Box<E>),

    LessEqual(Box<E>, Box<E>),
    GreaterEqual(Box<E>, Box<E>),
}

impl<E: Expression> Condition<E> {
    pub fn evaluate(&self, variables: &[E]) -> Result<bool, ExpressionError> {
        Ok(match *self {
            Condition::True => true,
            Condition::False => false,
            Condition::Not(ref c) => !try!(c.evaluate(variables)),
            Condition::And(ref c1, ref c2) => {
                try!(c1.evaluate(variables)) && try!(c2.evaluate(variables))
            }
            Condition::Or(ref c1, ref c2) => {
                try!(c1.evaluate(variables)) || try!(c2.evaluate(variables))
            }
            Condition::Equal(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) == try!(e2.evaluate(variables))
            }
            Condition::Less(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) < try!(e2.evaluate(variables))
            }
            Condition::Greater(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) > try!(e2.evaluate(variables))
            }
            Condition::LessEqual(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) <= try!(e2.evaluate(variables))
            }
            Condition::GreaterEqual(ref e1, ref e2) => {
                try!(e1.evaluate(variables)) >= try!(e2.evaluate(variables))
            }
        })
    }
}

impl<'a, E, T> Into<Sexp> for &'a Condition<E>
    where E: Expression<Element = T>,
          &'a E: Into<Sexp>,
          T: ElementType + Into<Sexp>
{
    fn into(self) -> Sexp {
        match self {
            &Condition::True => Sexp::from("true"),
            &Condition::False => Sexp::from("false"),
            &Condition::Not(ref a) => Sexp::from(("not", Into::<Sexp>::into(a.as_ref()))),
            &Condition::And(ref a, ref b) => {
                Sexp::from(("and",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Condition::Or(ref a, ref b) => {
                Sexp::from(("or",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Condition::Equal(ref a, ref b) => {
                Sexp::from(("==",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Condition::Less(ref a, ref b) => {
                Sexp::from(("<",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Condition::Greater(ref a, ref b) => {
                Sexp::from((">",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Condition::LessEqual(ref a, ref b) => {
                Sexp::from(("<=",
                            Into::<Sexp>::into(a.as_ref()),
                            Into::<Sexp>::into(b.as_ref())))
            }
            &Condition::GreaterEqual(ref a, ref b) => {
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
    let cond = Condition::Greater(box NumExpr::Var(0), box NumExpr::Const(0.0));

    fn fun(a: f32) -> bool {
        a > 0.0
    }

    fn check(cond: &Condition<NumExpr<f32>>, a: f32) {
        assert_eq!(Ok(fun(a)), cond.evaluate(&[NumExpr::Const(a)]))
    }

    check(&cond, 123.0);
    check(&cond, 0.0);
    check(&cond, -1.4);

    let no_vars: &[NumExpr<f32>] = &[];
    assert_eq!(Ok(true), Condition::True.evaluate(no_vars));
}
