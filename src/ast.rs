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



impl fmt::Display for Num {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let top_units = self.units.iter().filter(|(_,v)| **v > 0);
        let mut bottom_units = self.units.iter().filter(|(_,v)| **v < 0).peekable();

        let (upper, lower) = if self.val >= 1.0 || self.val <= -1.0 {
            (self.val, 1.0)
        } else {
            (1.0, 1.0/self.val)
        };

        write!(f, "{} ", upper)?;

        for (name, power) in top_units {
            write!(f, "{}", name)?;

            if *power != 1 {
                write!(f, "^{} ", power)?;
            } else {
                write!(f, " ")?;
            }
        }

        let has_bottom_units = bottom_units.peek().is_some();
        let has_bottom_number = lower != 1.0;

        if  has_bottom_units || has_bottom_number {
            write!(f, "/ ")?;
        }

        if has_bottom_number {
            write!(f, "{} ", lower)?;
        }


        for (name, power) in bottom_units {
            write!(f, "{}", name)?;

            if *power != -1 {
                write!(f, "^{} ", -power)?;
            } else {
                write!(f, " ")?;
            }
        }

        Ok(())
    }

}


pub fn pow(lhs: &Num, rhs: &Num)  -> Result<Num, String>  {
    if rhs.units.is_empty() {
        let units = lhs.units.iter().map(|(k,v)| (k.clone(), *v * (rhs.val as i32))).collect();
        Ok(Num{val: lhs.val.powf(rhs.val), units})
    } else {
        Err(format!("Can't raise {} to power with units {}", lhs, rhs))
    }
    
}


pub fn sum(lhs: &Num, rhs: &Num) -> Result<Num, String> {
    if lhs.units == rhs.units {
        Ok(Num{val: lhs.val + rhs.val, ..lhs.clone()})
    } else {
        Err(format!("Can't add disparate units {} and {}", lhs, rhs))
    }
}

pub fn sub(lhs: &Num, rhs: &Num) -> Result<Num, String> {
    if lhs.units == rhs.units {
        Ok(Num{val: lhs.val - rhs.val, ..lhs.clone()})
    } else {
        Err(format!("Can't subtract disparate units {} and {}", lhs, rhs))
    }
}

pub fn mul(lhs: &Num, rhs: &Num) -> Num {
        let mut units = lhs.units.clone();
        for (name, power) in &rhs.units {
            *units.entry(name.to_string()).or_insert(0) += power;
        }

        Num{val: lhs.val * rhs.val, units}
    }


pub fn div(lhs: &Num, rhs: &Num) -> Num {
    let mut units = lhs.units.clone();
    for (name, power) in &rhs.units {
        *units.entry(name.to_string()).or_insert(0) -= power;
    }

    Num{val: lhs.val / rhs.val, units}
}

