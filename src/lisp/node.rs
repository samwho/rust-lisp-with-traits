use std::{collections::HashMap, marker::PhantomData};

pub trait Node {
    type Return;
    fn eval(self) -> Self::Return;
}

macro_rules! identity_node {
    ( $($t:ty),* ) => {
      $(
        impl Node for $t {
            type Return = $t;

            fn eval(self) -> Self::Return {
                self
            }
        }
      )*
    };
}

identity_node!(char, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, String);

impl<F, R> Node for (F,)
where
    F: Fn() -> R,
{
    type Return = R;
    fn eval(self) -> Self::Return {
        self.0()
    }
}

impl<F, A, B, R> Node for (F, A)
where
    F: Fn(B) -> R,
    A: Node<Return = B>,
{
    type Return = R;
    fn eval(self) -> Self::Return {
        self.0(self.1.eval())
    }
}

impl<F, A1, A2, R1, R2, R> Node for (F, A1, A2)
where
    F: Fn(R1, R2) -> R,
    A1: Node<Return = R1>,
    A2: Node<Return = R2>,
{
    type Return = R;
    fn eval(self) -> Self::Return {
        self.0(self.1.eval(), self.2.eval())
    }
}

impl<F, A1, A2, A3, R1, R2, R3, R> Node for (F, A1, A2, A3)
where
    F: Fn(R1, R2, R3) -> R,
    A1: Node<Return = R1>,
    A2: Node<Return = R2>,
    A3: Node<Return = R3>,
{
    type Return = R;
    fn eval(self) -> Self::Return {
        self.0(self.1.eval(), self.2.eval(), self.3.eval())
    }
}

impl<F, A1, A2, A3, A4, R1, R2, R3, R4, R> Node for (F, A1, A2, A3, A4)
where
    F: Fn(R1, R2, R3, R4) -> R,
    A1: Node<Return = R1>,
    A2: Node<Return = R2>,
    A3: Node<Return = R3>,
    A4: Node<Return = R4>,
{
    type Return = R;
    fn eval(self) -> Self::Return {
        self.0(self.1.eval(), self.2.eval(), self.3.eval(), self.4.eval())
    }
}

impl<T> Node for Vec<T> {
    type Return = Self;
    fn eval(self) -> Self::Return {
        self
    }
}

impl<K, V> Node for HashMap<K, V> {
    type Return = Self;
    fn eval(self) -> Self::Return {
        self
    }
}

struct FAR<F, A, R>
where
    F: Fn(A) -> R,
{
    f: F,
    a: PhantomData<A>,
}

impl<F, A, R> From<F> for FAR<F, A, R>
where
    F: Fn(A) -> R,
{
    fn from(f: F) -> Self {
        FAR { f, a: PhantomData }
    }
}

impl<F, A, R> Node for FAR<F, A, R>
where
    F: Fn(A) -> R,
{
    type Return = F;

    fn eval(self) -> Self::Return {
        self.f
    }
}

struct FAAR<F, A1, A2, R>
where
    F: Fn(A1, A2) -> R,
{
    f: F,
    a1: PhantomData<A1>,
    a2: PhantomData<A2>,
}

impl<F, A1, A2, R> From<F> for FAAR<F, A1, A2, R>
where
    F: Fn(A1, A2) -> R,
{
    fn from(f: F) -> Self {
        FAAR {
            f,
            a1: PhantomData,
            a2: PhantomData,
        }
    }
}

impl<F, A1, A2, R> Node for FAAR<F, A1, A2, R>
where
    F: Fn(A1, A2) -> R,
{
    type Return = F;

    fn eval(self) -> Self::Return {
        self.f
    }
}
