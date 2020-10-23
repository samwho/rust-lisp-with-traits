use lisp::prelude::*;

fn add_one(a: i32) -> i32 {
    a + 1
}

fn main() {
    let v = vec![1, 2, 3];
    let add_one: fn(i32) -> i32 = add_one;
    let res = eval((map, add_one, v));
    // let res = eval((reduce, 0, add, (map, add_one, vec![1, 2, 3])));
    println!("{:?}", res);
}
