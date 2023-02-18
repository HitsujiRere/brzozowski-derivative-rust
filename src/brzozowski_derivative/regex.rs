use std::fmt;

/// 正規表現
#[derive(Debug, Clone, PartialEq)]
pub enum Regex {
    EmptySet,
    Epsilon,
    Symbol(char),
    Concat(Box<Regex>, Box<Regex>),
    Union(Box<Regex>, Box<Regex>),
    Star(Box<Regex>),
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Regex::*;
        match self {
            EmptySet => write!(f, "∅"),
            Epsilon => write!(f, "ε"),
            Symbol(ch) => write!(f, "{}", ch),
            Concat(x, y) => {
                match **x {
                    EmptySet | Epsilon | Symbol(_) | Concat(_, _) | Star(_) => write!(f, "{}", x)?,
                    Union(_, _) => write!(f, "({})", x)?,
                };
                match **y {
                    EmptySet | Epsilon | Symbol(_) | Concat(_, _) | Star(_) => write!(f, "{}", y)?,
                    Union(_, _) => write!(f, "({})", y)?,
                };
                Ok(())
            }
            Union(x, y) => write!(f, "{}|{}", x, y),
            Star(r) => match **r {
                EmptySet | Epsilon | Symbol(_) | Star(_) => write!(f, "{}*", r),
                Concat(_, _) | Union(_, _) => write!(f, "({})*", r),
            },
        }
    }
}
