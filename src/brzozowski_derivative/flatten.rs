use super::Regex;

impl Regex {
    /// 平らにする
    pub fn flatten(&self) -> Regex {
        use Regex::*;
        match self {
            EmptySet => EmptySet,
            Epsilon => Epsilon,
            Symbol(symbol) => Symbol(*symbol),
            Concat(x, y) => match **x {
                EmptySet => EmptySet,
                Epsilon => *y.clone(),
                _ => match **y {
                    EmptySet => EmptySet,
                    Epsilon => *x.clone(),
                    _ => Concat(Box::new(*x.clone()), Box::new(*y.clone())),
                },
            },
            Union(x, y) => match **x {
                EmptySet => *y.clone(),
                Epsilon => match **y {
                    EmptySet => Epsilon,
                    Epsilon => Epsilon,
                    _ => Union(Box::new(*x.clone()), Box::new(*y.clone())),
                },
                _ => match **y {
                    EmptySet => *x.clone(),
                    _ => Union(Box::new(*x.clone()), Box::new(*y.clone())),
                },
            },
            Star(r) => match **r {
                EmptySet => Epsilon,
                Epsilon => Epsilon,
                _ => Star(Box::new(r.flatten())),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_emptyset() {
        use Regex::*;
        assert_eq!(EmptySet.flatten(), EmptySet);
    }

    #[test]
    fn flatten_epsilon() {
        use Regex::*;
        assert_eq!(Epsilon.flatten(), Epsilon);
    }

    #[test]
    fn flatten_symbol() {
        use Regex::*;
        assert_eq!(Symbol('a').flatten(), Symbol('a'));
    }

    #[test]
    fn flatten_concat() {
        use Regex::*;
        assert_eq!(
            Concat(Box::new(EmptySet), Box::new(EmptySet)).flatten(),
            EmptySet
        );
        assert_eq!(
            Concat(Box::new(Epsilon), Box::new(Epsilon)).flatten(),
            Epsilon
        );
        assert_eq!(
            Concat(Box::new(EmptySet), Box::new(Epsilon)).flatten(),
            EmptySet
        );
        assert_eq!(
            Concat(Box::new(Epsilon), Box::new(EmptySet)).flatten(),
            EmptySet
        );
        assert_eq!(
            Concat(Box::new(Symbol('a')), Box::new(EmptySet)).flatten(),
            EmptySet
        );
        assert_eq!(
            Concat(Box::new(EmptySet), Box::new(Symbol('a'))).flatten(),
            EmptySet
        );
        assert_eq!(
            Concat(Box::new(Symbol('a')), Box::new(Epsilon)).flatten(),
            Symbol('a')
        );
        assert_eq!(
            Concat(Box::new(Epsilon), Box::new(Symbol('a'))).flatten(),
            Symbol('a')
        );
    }

    #[test]
    fn flatten_union() {
        use Regex::*;
        assert_eq!(
            Union(Box::new(EmptySet), Box::new(EmptySet)).flatten(),
            EmptySet
        );
        assert_eq!(
            Union(Box::new(Epsilon), Box::new(Epsilon)).flatten(),
            Epsilon
        );
        assert_eq!(
            Union(Box::new(Epsilon), Box::new(EmptySet)).flatten(),
            Epsilon
        );
        assert_eq!(
            Union(Box::new(EmptySet), Box::new(Epsilon)).flatten(),
            Epsilon
        );
        assert_eq!(
            Union(Box::new(Symbol('a')), Box::new(EmptySet)).flatten(),
            Symbol('a')
        );
        assert_eq!(
            Union(Box::new(EmptySet), Box::new(Symbol('a'))).flatten(),
            Symbol('a')
        );
        assert_eq!(
            Union(Box::new(Symbol('a')), Box::new(Epsilon)).flatten(),
            Union(Box::new(Symbol('a')), Box::new(Epsilon))
        );
        assert_eq!(
            Union(Box::new(Epsilon), Box::new(Symbol('a'))).flatten(),
            Union(Box::new(Epsilon), Box::new(Symbol('a')))
        );
    }

    #[test]
    fn flatten_star() {
        use Regex::*;
        assert_eq!(Star(Box::new(EmptySet)).flatten(), Epsilon);
        assert_eq!(Star(Box::new(Epsilon)).flatten(), Epsilon);
    }
}
