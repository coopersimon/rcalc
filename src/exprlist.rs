use super::expr::Expr;
use super::number::Base;
use super::state::State;

pub enum ExprList {
    Node{
        next: Box<ExprList>,
        expr: Box<Expr>
    },
    Null
}

impl ExprList {
    pub fn eval(&self, state: &mut State, base: &Base) -> String {
        fn eval_rec(expr_list: &ExprList, mut results: Vec<String>, state: &mut State, base: &Base) -> String {
            use self::ExprList::*;

            match expr_list {
                Node{ next: n, expr: e } => match e.eval(state) {
                    Ok(v) => {
                        results.push(v.base_string(&base));
                        state.set_ans(v);
                        eval_rec(&n, results, state, base)
                    },
                    Err(e) => {
                        results.push(format!("Error: {}", e));
                        results.join("; ")
                    }
                },
                Null => results.join("; ")
            }
        }

        eval_rec(self, Vec::new(), state, base)
    }
}