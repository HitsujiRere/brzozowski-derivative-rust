mod brzozowski_derivative;

use brzozowski_derivative::{parse, Regex};

fn main() {
    let r = Regex::Symbol('a');
    eprintln!("{:?}", r);
    eprintln!("{:?}", r.derivative('a'));

    eprintln!("{:?}", parse("ab*|cd"))
}
