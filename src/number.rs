use std::{
    ops::{
        Add,
        Sub,
        Mul,
        Div,
        Neg,
        Rem
    },
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    }
};

use super::error::RuntimeError;

pub type Itype = i64;
pub type Ftype = f64;

#[derive(Debug)]
pub enum Base {
    Decimal(Option<usize>),
    Binary(Option<usize>),
    Octal(Option<usize>),
    Hexadecimal(Option<usize>)
}

pub type NumberResult = Result<Number, RuntimeError>;

#[derive(Clone)]
pub enum Number {
    Integer(Itype),
    Fraction(Ftype)
}

impl Number {
    pub fn base_string(&self, base: &Base) -> String {
        use Number::*;
        use Base::*;
        match self {
            Integer(i) => match base {
                Decimal(p)      => if let Some(p) = p {
                    format!("{:1$}", i, p)
                } else {
                    format!("{}", i)
                },
                Binary(p)       => if let Some(p) = p {
                    format!("{:#01$b}", i, p+2)
                } else {
                    format!("{:#b}", i)
                },
                Octal(p)        => if let Some(p) = p {
                    format!("{:#01$o}", i, p+2)
                } else {
                    format!("{:#o}", i)
                },
                Hexadecimal(p)  => if let Some(p) = p {
                    format!("{:#01$X}", i, p+2)
                } else {
                    format!("{:#X}", i)
                }
            },
            Fraction(f) => {
                match base {
                    Decimal(p)  => if let Some(p) = p {
                        return format!("{:.*}", p, f);
                    },
                    _   => ()
                }
                format!("{}", f)
            }
        }
    }

    pub fn pow(&self, to_power: Number) -> NumberResult {
        use Number::*;
        Ok(match self {
            Integer(l) => match to_power {
                Integer(r) => Integer(l.pow(r as u32)),
                Fraction(r) => Fraction((*l as f64).powf(r)),
            },
            Fraction(l) => match to_power {
                Integer(r) => Fraction(l.powi(r as i32)),
                Fraction(r) => Fraction(l.powf(r)),
            }
        })
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Number::*;
        match self {
            Integer(i) => write!(f, "{}", i),
            Fraction(d) => write!(f, "{}", d)
        }
    }
}

trait Operator {
    fn i_op(l: Itype, r: Itype) -> Itype;
    fn f_op(l: Ftype, r: Ftype) -> Ftype;
}

macro_rules! binary_operator {
    ( $x:tt ) => {
        fn i_op(l: Itype, r: Itype) -> Itype {
            l $x r
        }

        fn f_op(l: Ftype, r: Ftype) -> Ftype {
            l $x r
        }
    };
}

fn operate<O: Operator>(left: Number, right: Number) -> Number {
    use Number::*;
    match (left, right) {
        (Integer(l), Integer(r))    => Integer(O::i_op(l, r)),
        (Integer(l), Fraction(r))   => Fraction(O::f_op(l as Ftype, r)),
        (Fraction(l), Integer(r))   => Fraction(O::f_op(l, r as Ftype)),
        (Fraction(l), Fraction(r))  => Fraction(O::f_op(l, r)),
    }
}

struct AddOperator;
impl Operator for AddOperator {
    binary_operator!(+);
}

impl Add for Number {
    type Output = NumberResult;

    fn add(self, other: Self) -> Self::Output {
        Ok(operate::<AddOperator>(self, other))
    }
}

struct SubOperator;
impl Operator for SubOperator {
    binary_operator!(-);
}

impl Sub for Number {
    type Output = NumberResult;

    fn sub(self, other: Self) -> Self::Output {
        Ok(operate::<SubOperator>(self, other))
    }
}

struct MulOperator;
impl Operator for MulOperator {
    binary_operator!(*);
}

impl Mul for Number {
    type Output = NumberResult;

    fn mul(self, other: Self) -> Self::Output {
        Ok(operate::<MulOperator>(self, other))
    }
}

impl Div for Number {
    type Output = NumberResult;

    fn div(self, other: Self) -> Self::Output {
        use Number::*;
        match other {
            Integer(0)  => Err(RuntimeError::DivByZero),
            other       => {
                let ans = match (self, other) {
                    (Integer(l), Integer(r)) => {
                        let d = l as Ftype / r as Ftype;
                        let i = l / r;
                        if i as Ftype == d {
                            Integer(i)
                        } else {
                            Fraction(d)
                        }
                    },
                    (Integer(l), Fraction(r))   => Fraction(l as Ftype / r),
                    (Fraction(l), Integer(r))   => Fraction(l / r as Ftype),
                    (Fraction(l), Fraction(r))  => Fraction(l / r),
                };
                match ans {
                    Fraction(f) => if f.is_finite() { Ok(ans) } else { Err(RuntimeError::NaNFloatResult) },
                    other       => Ok(other)
                }
            }
        }
    }
}

impl Rem for Number {
    type Output = NumberResult;

    fn rem(self, other: Self) -> Self::Output {
        use Number::*;
        match other {
            Integer(0)  => Err(RuntimeError::DivByZero),
            other       => {
                let ans = match (self, other) {
                    (Integer(l), Integer(r)) => {
                        let d = l as Ftype % r as Ftype;
                        let i = l % r;
                        if i as Ftype == d {
                            Integer(i)
                        } else {
                            Fraction(d)
                        }
                    },
                    (Integer(l), Fraction(r))   => Fraction(l as Ftype % r),
                    (Fraction(l), Integer(r))   => Fraction(l % r as Ftype),
                    (Fraction(l), Fraction(r))  => Fraction(l % r),
                };
                match ans {
                    Fraction(f) => if f.is_finite() { Ok(ans) } else { Err(RuntimeError::NaNFloatResult) },
                    other       => Ok(other)
                }
            }
        }
    }
}

impl Neg for Number {
    type Output = NumberResult;

    fn neg(self) -> Self::Output {
        use Number::*;

        Ok(match self {
            Integer(i)  => Integer(-i),
            Fraction(f) => Fraction(-f)
        })
    }
}