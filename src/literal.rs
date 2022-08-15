use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Clone, PartialEq, Debug)]
pub enum Literal {
    Number(f64),
    String(String),
    Unit,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(n) => {
                write!(f, "{n}")
            }
            Literal::String(s) => {
                write!(f, "{s}")
            }
            Literal::Unit => {
                write!(f, "")
            }
        }
    }
}

impl Add for Literal {
    type Output = Result<Self, String>;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Number(v) => match rhs {
                Literal::Number(n) => Ok(Literal::Number(v + n)),
                _ => Err("Failed to Add".to_string()),
            },
            Literal::String(v) => match rhs {
                Literal::String(s) => Ok(Literal::String(v + &s)),
                _ => Err("Failed to Add".to_string()),
            },
            _ => Err("Failed to Add".to_string()),
        }
    }
}

impl Sub for Literal {
    type Output = Result<Self, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Number(v) => match rhs {
                Literal::Number(n) => Ok(Literal::Number(v - n)),
                _ => Err("Failed to Sub".to_string()),
            },
            _ => Err("Failed to Sub".to_string()),
        }
    }
}

impl Mul for Literal {
    type Output = Result<Self, String>;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Number(v) => match rhs {
                Literal::Number(n) => Ok(Literal::Number(v * n)),
                _ => Err("Failed to Mul".to_string()),
            },
            _ => Err("Failed to Mul".to_string()),
        }
    }
}

impl Div for Literal {
    type Output = Result<Self, String>;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Number(v) => match rhs {
                Literal::Number(n) => Ok(Literal::Number(v / n)),
                _ => Err("Failed to Div".to_string()),
            },
            _ => Err("Failed to Div".to_string()),
        }
    }
}

impl Neg for Literal {
    type Output = Result<Self, String>;
    fn neg(self) -> Self::Output {
        match self {
            Literal::Number(n) => Ok(Literal::Number(-n)),
            _ => Err("Failed to Neg".to_string()),
        }
    }
}
