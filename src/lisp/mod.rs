mod iter;
mod math;
mod node;

pub use iter::*;
pub use math::*;
pub use node::*;

pub fn eval<N, R>(n: N) -> R
where
    N: Node<Return = R>,
{
    n.eval()
}
