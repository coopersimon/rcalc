mod error;
mod expr;
mod number;
mod state;

use lalrpop_util::lalrpop_mod;
use clap::{clap_app, crate_version};

use std::io;

use state::State;

lalrpop_mod!(pub rcalc);

fn main() {
    let app = clap_app!(calc =>
        (version: crate_version!())
        (author: "Simon Cooper")
        (about: "Calculator used for simple maths and base conversion.")
        (@arg EXPR: "The expression to evaluate. If this is omitted, calc launches an interpreter.")
    );

    let cmd_args = app.get_matches();

    if let Some(expr) = cmd_args.value_of("EXPR") {
        let parser = rcalc::ExprParser::new();
        let expr = parser.parse(expr).unwrap();

        let mut state = State::new();

        match expr.eval(&mut state) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("Error: {}", e)
        }
    } else {
        interpret();
    }
}

fn interpret() {
    let parser = rcalc::ExprParser::new();
    let mut state = State::new();

    println!("calc interpret");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read from stdin.");

        let trimmed = input.trim();

        if trimmed == "q" || trimmed == "quit" {
            break;
        } else {
            let expr = parser.parse(trimmed).unwrap();
            match expr.eval(&mut state) {
                Ok(v) => {
                    println!("\t= {}", v);
                    state.set_ans(v);
                },
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }
}