mod expr;
mod number;

use lalrpop_util::lalrpop_mod;
use clap::clap_app;

lalrpop_mod!(pub rcalc);

fn main() {
    let parser = rcalc::ExprParser::new();

    let cmd_args = clap_app!(calc =>
        (version: "0.1.0")
        (author: "Simon Cooper")
        (about: "Calculator used for simple maths and base conversion.")
        (@arg EXPR: "The expression to evaluate. If this is omitted, calc launches an interpreter.")
    ).get_matches();

    if let Some(expr) = cmd_args.value_of("EXPR") {
        println!("{}", parser.parse(expr).unwrap().eval());
    } else {
        println!("No argument provided.");
    }
}