use lisp::prelude::*;

fn add_one(a: i32) -> i32 {
    a + 1
}

fn main() {
    // Needs to be converted to function pointer
    let add_one: fn(_) -> _ = add_one;
    // let add_one = lambda(|a| a + 1);
    // let add_one = lambda(add_one);
    // let res = eval((map, add_one, vec![1, 2, 3]));

    let add: fn(_, _) -> _ = add;
    // let add = lambda2(add);
    let res = eval((reduce, 0, add, (map, add_one, vec![1, 2, 3])));
    // let res = eval((reduce, lazy(|| 0), add, (map, add_one, vec![1, 2, 3])));

    // The following is an alternative that uses reduce2 and allows defining
    // lambdas that accept any number of arguments by just using `lambda`.
    // let add = lambda(|(r, e)| r + e);
    // let res = eval((reduce2, 0, add, (map, add_one, vec![1, 2, 3])));
    println!("{:?}", res);
}
