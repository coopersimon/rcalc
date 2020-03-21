use super::error::RuntimeError;
use super::number::{NumberResult, Number};
use super::state::State;

use std::ops::Neg;

pub enum Expr {
    Ans,
    Num(Number),
    Var(String),
    SetVar(String, Box<Expr>),
    
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>)
}

impl Expr {
    pub fn eval(&self, state: &mut State) -> NumberResult {
        use self::Expr::*;
        match self {
            SetVar(v, e)    => {
                let val = e.eval(state)?;
                state.set_var(&v, val.clone());
                Ok(val)
            },
            Ans         => state.get_ans().ok_or(RuntimeError::AnsNotDefined),
            Num(n)      => Ok(n.clone()),
            Var(v)      => state.get_var(&v).ok_or(RuntimeError::VarNotDefined(v.clone())),
            Neg(e)      => e.eval(state)?.neg(),
            Add(l, r)   => l.eval(state)? + r.eval(state)?,
            Sub(l, r)   => l.eval(state)? - r.eval(state)?,
            Mul(l, r)   => l.eval(state)? * r.eval(state)?,
            Div(l, r)   => l.eval(state)? / r.eval(state)?,
            Mod(l, r)   => l.eval(state)? % r.eval(state)?,
            Pow(l, r)   => l.eval(state)?.pow(r.eval(state)?),
        }
    }
}