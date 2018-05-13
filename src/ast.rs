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

use std;
use std::collections::HashMap;
use std::fmt;

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
    Num(Num),
    Exp(Box<Expression>),
    NPowE(Num, Box<Expression>),
    EPowN(Box<Expression>, Num),
    EPowE(Box<Expression>, Box<Expression>),
}





#[derive(Debug, Clone)]
pub struct Num {
    pub val: f64,
    pub units: HashMap<String, i32>,
}

impl Num {
    pub fn pow(&self, other: &Num) -> Num {
        assert!(other.units.is_empty());
        let units = self.units.iter().map(|(k,v)| (k.clone(), *v * (other.val as i32))).collect();
        Num{val: self.val.powf(other.val), units}
    }
}

impl<'a> std::ops::Add for &'a Num {
    type Output = Num;

    fn add(self, other: &Num) -> Num {
        assert_eq!(self.units, other.units);
        Num{val: self.val + other.val, ..self.clone()}
    }
}


impl<'a> std::ops::Sub for &'a Num {
    type Output = Num;

    fn sub(self, other: &Num) -> Num {
        assert_eq!(self.units, other.units);
        Num{val: self.val - other.val, ..self.clone()}
    }
}

impl<'a> std::ops::Mul for &'a Num {
    type Output = Num;

    fn mul(self, other: &Num) -> Num {
        let mut units = self.units.clone();
        for (name, power) in &other.units {
            *units.entry(name.to_string()).or_insert(0) += power;
        }

        Num{val: self.val * other.val, units}
    }
}

impl<'a> std::ops::Div for &'a Num {
    type Output = Num;

    fn div(self, other: &Num) -> Num {
        let mut units = self.units.clone();
        for (name, power) in &other.units {
            *units.entry(name.to_string()).or_insert(0) -= power;
        }

        Num{val: self.val / other.val, units}
    }
}


impl fmt::Display for Num {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let top_units = self.units.iter().filter(|(_,v)| **v > 0);
        let mut bottom_units = self.units.iter().filter(|(_,v)| **v < 0).peekable();

        write!(f, "{} ", self.val)?;

        for (name, power) in top_units {
            write!(f, "{}", name)?;

            if *power != 1 {
                write!(f, "^{} ", power)?;
            } else {
                write!(f, " ")?;
            }
        }


        if bottom_units.peek().is_some() {
            write!(f, "/ ")?;

            for (name, power) in bottom_units {
                write!(f, "{}", name)?;

                if *power != 1 {
                    write!(f, "^{} ", power)?;
                } else {
                    write!(f, " ")?;
                }
            } 
        }

        Ok(())
    }

}
