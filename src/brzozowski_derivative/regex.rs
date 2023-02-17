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
            Concat(x, y) => write!(f, "{}{}", x, y),
            Union(x, y) => write!(f, "{}|{}", x, y),
            Star(r) => write!(f, "{}*", r),
        }
    }
}
