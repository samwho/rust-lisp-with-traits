use std::iter::Map;

pub fn map<I, E, F, R>(f: F, i: I) -> Map<I::IntoIter, F>
where
    F: FnMut(E) -> R,
    I: IntoIterator<Item = E>,
{
    i.into_iter().map(f)
}

pub fn reduce<I, E, F, R>(init: R, f: F, i: I) -> R
where
    F: FnMut(R, E) -> R,
    I: IntoIterator<Item = E>,
{
    i.into_iter().fold(init, f)
}

pub fn reduce2<I, E, F, R>(init: R, mut f: F, i: I) -> R
where
    F: FnMut((R, E)) -> R,
    I: IntoIterator<Item = E>,
{
    i.into_iter().fold(init, |r, e| f((r, e)))
}

pub fn to_vec<T, I>(i: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    i.into_iter().collect()
}
