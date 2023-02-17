use super::Regex;

impl Regex {
    /// 微分する
    pub fn derivative(&self, ch: char) -> Regex {
        use Regex::*;
        match self {
            EmptySet => EmptySet,
            Epsilon => EmptySet,
            Symbol(symbol) => {
                if ch == *symbol {
                    Epsilon
                } else {
                    EmptySet
                }
            }
            Concat(x, y) => Union(
                Box::new(
                    Concat(
                        Box::new(if x.is_accept_empty_str() {
                            Epsilon
                        } else {
                            EmptySet
                        }),
                        Box::new(y.derivative(ch)),
                    )
                    .flatten(),
                ),
                Box::new(Concat(Box::new(x.derivative(ch)), Box::new(*y.clone())).flatten()),
            )
            .flatten(),
            Union(x, y) => Union(Box::new(x.derivative(ch)), Box::new(y.derivative(ch))).flatten(),
            Star(r) => Concat(Box::new(r.derivative(ch)), Box::new(self.clone())).flatten(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivative_emptyset() {
        use Regex::*;
        assert_eq!(EmptySet.derivative('a'), EmptySet);
    }

    #[test]
    fn derivative_epsilon() {
        use Regex::*;
        assert_eq!(Epsilon.derivative('a'), EmptySet);
    }

    #[test]
    fn derivative_symbol() {
        use Regex::*;
        assert_eq!(Symbol('a').derivative('a'), Epsilon);
        assert_eq!(Symbol('a').derivative('b'), EmptySet);
    }

    #[test]
    fn derivative_concat() {
        use Regex::*;
        assert_eq!(
            Concat(Box::new(Symbol('a')), Box::new(Symbol('b'))).derivative('a'),
            Symbol('b')
        );
        assert_eq!(
            Concat(Box::new(Symbol('a')), Box::new(Symbol('b'))).derivative('b'),
            EmptySet
        );
        assert_eq!(
            Concat(
                Box::new(Concat(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('a'),
            Concat(Box::new(Symbol('b')), Box::new(Symbol('c')))
        );
        assert_eq!(
            Concat(
                Box::new(Concat(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('b'),
            EmptySet
        );
        assert_eq!(
            Concat(
                Box::new(Symbol('a')),
                Box::new(Concat(Box::new(Symbol('b')), Box::new(Symbol('c'))))
            )
            .derivative('a'),
            Concat(Box::new(Symbol('b')), Box::new(Symbol('c')))
        );
    }

    #[test]
    fn derivative_union() {
        use Regex::*;
        assert_eq!(
            Union(Box::new(Symbol('a')), Box::new(Symbol('b'))).derivative('a'),
            Epsilon
        );
        assert_eq!(
            Union(Box::new(Symbol('a')), Box::new(Symbol('b'))).derivative('b'),
            Epsilon
        );
        assert_eq!(
            Union(
                Box::new(Union(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('a'),
            Epsilon
        );
        assert_eq!(
            Union(
                Box::new(Union(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('b'),
            Epsilon
        );
        assert_eq!(
            Union(
                Box::new(Union(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('c'),
            Epsilon
        );
    }

    #[test]
    fn derivative_concat_union() {
        use Regex::*;
        assert_eq!(
            Concat(
                Box::new(Union(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('a'),
            Symbol('c')
        );
        assert_eq!(
            Concat(
                Box::new(Union(Box::new(Symbol('a')), Box::new(Symbol('b')))),
                Box::new(Symbol('c'))
            )
            .derivative('b'),
            Symbol('c')
        );
    }

    #[test]
    fn derivative_star() {
        use Regex::*;
        assert_eq!(
            Star(Box::new(Symbol('a'))).derivative('a'),
            Star(Box::new(Symbol('a')))
        );
        assert_eq!(Star(Box::new(Symbol('a'))).derivative('b'), EmptySet);
    }
}
