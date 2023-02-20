use std::fmt;

/// 正規表現
#[derive(Debug, Clone, PartialEq)]
pub enum Regex {
    /// 空集合 ∅
    EmptySet,
    // 空文字列 ε
    Epsilon,
    // 文字
    Symbol(char),
    // 連結
    Concat(Box<Regex>, Box<Regex>),
    // 和集合
    Union(Box<Regex>, Box<Regex>),
    // クリーネ閉包
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
