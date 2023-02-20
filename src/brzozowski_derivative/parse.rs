use super::Regex;
use nom::{
    branch::alt,
    character::complete::{char, none_of, one_of},
    combinator::{map, opt},
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn parse(input: &str) -> Result<Regex, String> {
    match expr(input) {
        Ok((_, regex)) => Ok(regex),
        Err(err) => Err(err.to_string()),
    }
}

// expr     = union
// union    = union ( "|" union )*
// concat   = length concat?
// length   = ident "*"?
// ident    = paren | symbol
// paren    = "(" expr ")"
// symbol   = .

fn expr(input: &str) -> IResult<&str, Regex> {
    union(input)
}

fn union(input: &str) -> IResult<&str, Regex> {
    map(
        pair(concat, opt(preceded(char('|'), union))),
        |(left, right)| match right {
            Some(right) => Regex::Union(Box::new(left), Box::new(right)),
            None => left,
        },
    )(input)
}

fn concat(input: &str) -> IResult<&str, Regex> {
    map(pair(length, opt(concat)), |(left, right)| match right {
        Some(right) => Regex::Concat(Box::new(left), Box::new(right)),
        None => left,
    })(input)
}

fn length(input: &str) -> IResult<&str, Regex> {
    map(pair(ident, opt(one_of("*+?"))), |(regex, length)| match length {
        Some(ch) => match ch {
            '*' => Regex::Star(Box::new(regex)),
            '+' => Regex::Concat(Box::new(regex.clone()), Box::new(Regex::Star(Box::new(regex)))),
            '?' => Regex::Union(Box::new(regex), Box::new(Regex::Epsilon)),
            _ => unreachable!(),
        },
        None => regex,
    })(input)
}

fn ident(input: &str) -> IResult<&str, Regex> {
    alt((paren, symbol))(input)
}

fn paren(input: &str) -> IResult<&str, Regex> {
    delimited(char('('), expr, char(')'))(input)
}

fn symbol(input: &str) -> IResult<&str, Regex> {
    map(none_of("()|*+?"), |ch: char| Regex::Symbol(ch))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        use Regex::*;
        assert_eq!(length("a*"), Ok(("", Star(Box::new(Symbol('a'))))));
        assert_eq!(length("a+"), Ok(("", Concat(Box::new(Symbol('a')), Box::new(Star(Box::new(Symbol('a'))))))));
        assert_eq!(length("a?"), Ok(("", Union(Box::new(Symbol('a')), Box::new(Epsilon)))));
    }

    #[test]
    fn test_paren() {
        use Regex::*;
        assert_eq!(paren("(a)|"), Ok(("|", Symbol('a'))));
    }

    #[test]
    fn test_concat() {
        use Regex::*;
        assert_eq!(
            concat("ab|"),
            Ok(("|", Concat(Box::new(Symbol('a')), Box::new(Symbol('b')))))
        );
        assert_eq!(
            concat("abc|"),
            Ok((
                "|",
                Concat(
                    Box::new(Symbol('a')),
                    Box::new(Concat(Box::new(Symbol('b')), Box::new(Symbol('c'))))
                )
            ))
        );
    }

    #[test]
    fn test_symbol() {
        use Regex::*;
        assert_eq!(symbol("a"), Ok(("", Symbol('a'))));
    }
}
