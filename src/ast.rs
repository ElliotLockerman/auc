/*
Modern Compilers in C p.45
E = expression
F = factor (things you multiply)
T = term (things you add)

E -> E + T
E -> E - T
E -> T

T -> T * F
T -> T / F
T -> F

F -> num
F -> ( E )

*/

#[derive(Debug, Clone)]
pub enum Expression {
    Sum(Box<Expression>, Box<Term>),
    Diff(Box<Expression>, Box<Term>),
    Term(Box<Term>),
}

#[derive(Debug, Clone)]
pub enum Term {
    Prod(Box<Term>, Box<Factor>),
    Quot(Box<Term>, Box<Factor>),
    Factor(Box<Factor>),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Num(f64),
    Exp(Box<Expression>),
    NPowE(f64, Box<Expression>),
    EPowN(Box<Expression>, f64),
    EPowE(Box<Expression>, Box<Expression>),
}




