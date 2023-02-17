use crate::brzozowski_derivative::Regex;

mod brzozowski_derivative;

fn main() {
    let r = Regex::Symbol('a');
    eprintln!("{:?}", r);
    eprintln!("{:?}", r.derivative('a'));
}
