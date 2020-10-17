use lisp::prelude::*;

fn hello_world() {
    println!("Hello, world!");
}

fn main() {
    eval((hello_world,));
}
