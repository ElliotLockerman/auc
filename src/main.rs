


extern crate rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub mod grammar;
pub mod ast;

use ast::*;


fn main() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if line.chars().into_iter().all(|ch| ch.is_whitespace()) {
                    continue; 
                }
                let exp = match grammar::ExpParser::new().parse(&line) {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    },
                };
       
                println!("{}", eval_exp(&exp));
            },
            Err(ReadlineError::Interrupted)| Err(ReadlineError::Eof) => break,
            Err(err) => println!("Error: {:?}", err),
        }
    }
}



fn eval_exp(exp: &Expression) -> f64 {
    match exp {
        Expression::Sum(lhs, rhs) => eval_exp(lhs) + eval_term(rhs),
        Expression::Diff(lhs, rhs) => eval_exp(lhs) - eval_term(rhs),
        Expression::Term(term) => eval_term(term),
    }
}

fn eval_term(term: &Term) -> f64 {
    match term {
        Term::Prod(lhs, rhs) => eval_term(lhs) * eval_fact(rhs),
        Term::Quot(lhs, rhs) => eval_term(lhs) / eval_fact(rhs),
        Term::Factor(fact) => eval_fact(fact),
    }
}

fn eval_fact(fact: &Factor) -> f64 {
    match fact {
        Factor::Num(n) => *n,
        Factor::Exp(e) => eval_exp(e),
        Factor::NPowE(f, e) => f.powf(eval_exp(e)),
        Factor::EPowN(e, f) => eval_exp(e).powf(*f),
        Factor::EPowE(e1, e2) => eval_exp(e1).powf(eval_exp(e2)),
    }
}



