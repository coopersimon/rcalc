use super::number::Number;

pub enum Expr {
    Num(Number),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>)
}

impl Expr {
    pub fn eval(&self) -> Number {
        use self::Expr::*;
        match self {
            Num(n)      => n.clone(),
            Add(l, r)   => l.eval() + r.eval(),
            Sub(l, r)   => l.eval() - r.eval(),
            Mul(l, r)   => l.eval() * r.eval(),
            Div(l, r)   => l.eval() / r.eval(),
        }
    }
}