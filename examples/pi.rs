use std::f64::consts::PI;

use lisp::prelude::*;

fn main() {
    let r = 3.0;
    let res = eval((mul, PI, (mul, r, r)));
    println!("{}", res);
}
