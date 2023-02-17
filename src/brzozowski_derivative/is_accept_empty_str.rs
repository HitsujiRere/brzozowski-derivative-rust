use super::Regex;

impl Regex {
    pub fn is_accept_empty_str(&self) -> bool {
        use Regex::*;
        match self {
            EmptySet => false,
            Epsilon => true,
            Symbol(_) => false,
            Concat(x, y) => x.is_accept_empty_str() && y.is_accept_empty_str(),
            Union(x, y) => x.is_accept_empty_str() || y.is_accept_empty_str(),
            Star(_) => true,
        }
    }
}
