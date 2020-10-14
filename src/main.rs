use std::{f64::consts::PI, ops::Mul};

mod lisp;

fn main() {
    let r = 3.0;
    let res = lisp::eval((Mul::mul, PI, (Mul::mul, r, r)));
    println!("{}", res);
}
