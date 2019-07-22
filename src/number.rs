use std::{
    ops::{
        Add,
        Sub,
        Mul,
        Div
    },
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    }
};

pub type Itype = i64;
pub type Ftype = f64;

#[derive(Debug, Clone)]
pub enum Number {
    Integer(Itype),
    Fraction(Ftype)
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Number::*;
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
    type Output = Self;

    fn add(self, other: Self) -> Self {
        operate::<AddOperator>(self, other)
    }
}

struct SubOperator;
impl Operator for SubOperator {
    binary_operator!(-);
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        operate::<SubOperator>(self, other)
    }
}

struct MulOperator;
impl Operator for MulOperator {
    binary_operator!(*);
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        operate::<MulOperator>(self, other)
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        use Number::*;
        match (self, other) {
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
        }
    }
}
