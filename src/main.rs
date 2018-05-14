


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
            
                match eval_exp(&exp) {
                    Ok(num) => println!("{}", num),
                    Err(msg) => println!("{}", msg),
                }
                
            },
            Err(ReadlineError::Interrupted)| Err(ReadlineError::Eof) => break,
            Err(err) => println!("Error: {:?}", err),
        }
    }
}



fn eval_exp(exp: &Expression) -> Result<Num, String> {
    match exp {
        Expression::Sum(lhs, rhs) => sum(&eval_exp(lhs)?, &eval_term(rhs)?),
        Expression::Diff(lhs, rhs) => diff(&eval_exp(lhs)?, &eval_term(rhs)?),
        Expression::Term(term) => eval_term(term),
    }
}

fn eval_term(term: &Term) -> Result<Num, String> {
    match term {
        Term::Prod(lhs, rhs) => Ok(&eval_term(lhs)? * &eval_fact(rhs)?),
        Term::Quot(lhs, rhs) => Ok(&eval_term(lhs)? / &eval_fact(rhs)?),
        Term::Factor(fact) => eval_fact(fact),
    }
}

fn eval_fact(fact: &Factor) -> Result<Num, String> {
    match fact {
        Factor::Num(n) => Ok(n.clone()),
        Factor::Exp(e) => eval_exp(e),
        Factor::NPowE(n, e) => Ok(n.pow(&eval_exp(e)?)),
        Factor::EPowN(e, n) => Ok(eval_exp(e)?.pow(n)),
        Factor::EPowE(e1, e2) => Ok(eval_exp(e1)?.pow(&eval_exp(e2)?)),
    }
}




