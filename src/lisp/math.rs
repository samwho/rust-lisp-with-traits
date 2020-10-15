use std::ops::{Add, Div, Mul, Sub};

pub fn add<A, B, C>(a: A, b: B) -> C
where
    A: Add<B, Output = C>,
{
    a + b
}

pub fn sub<A, B, C>(a: A, b: B) -> C
where
    A: Sub<B, Output = C>,
{
    a - b
}

pub fn mul<A, B, C>(a: A, b: B) -> C
where
    A: Mul<B, Output = C>,
{
    a * b
}

pub fn div<A, B, C>(a: A, b: B) -> C
where
    A: Div<B, Output = C>,
{
    a / b
}
