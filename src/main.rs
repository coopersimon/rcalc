mod error;
mod expr;
mod exprlist;
mod number;
mod state;

use lalrpop_util::lalrpop_mod;
use clap::{clap_app, crate_version};

use std::io;

use number::Base;
use state::State;

lalrpop_mod!(pub rcalc);

fn main() {
    let app = clap_app!(calc =>
        (version: crate_version!())
        (author: "Simon Cooper")
        (about: "Calculator used for simple maths and base conversion.")
        (@arg EXPR: "The expression(s) to evaluate. If this is omitted, calc launches an interpreter.")
        (@group base =>
            (@arg bin: -b "Sets the output to binary.")
            (@arg oct: -o "Sets the output to octal.")
            (@arg hex: -h "Sets the output to hexadecimal.")
        )
        (@arg precision: -p +takes_value "Sets the precision of the answer.")
    );

    let cmd_args = app.get_matches();

    let precision = cmd_args.value_of("precision").map(|s| s.parse::<usize>().expect("Couldn't parse precision value."));
    let base = if cmd_args.is_present("bin") {
        Base::Binary(precision)
    } else if cmd_args.is_present("hex") {
        Base::Hexadecimal(precision)
    } else if cmd_args.is_present("oct") {
        Base::Octal(precision)
    } else {
        Base::Decimal(precision)
    };

    if let Some(expr) = cmd_args.value_of("EXPR") {
        let parser = rcalc::ExprListParser::new();
        let expr_list = parser.parse(expr).unwrap();

        let mut state = State::new();

        println!("{}", expr_list.eval(&mut state, &base));
    } else {
        interpret(&base);
    }
}

fn interpret(init_base: &Base) {
    let parser = rcalc::ExprListParser::new();
    let mut state = State::new();

    println!("calc interpret: Type 'q' or 'quit' to exit.");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read from stdin.");

        let trimmed = input.trim();

        if trimmed == "q" || trimmed == "quit" {
            break;
        } else {
            let expr = parser.parse(trimmed).unwrap();
            println!("\t= {}", expr.eval(&mut state, init_base));
        }
    }
}