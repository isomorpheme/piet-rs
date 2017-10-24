pub type Coords = (usize, usize);

pub fn map_pair<T, U, F>(pair: (T, T), func: F) -> (U, U)
    where F: Fn(T) -> U
{
    (func(pair.0), func(pair.1))
}

pub fn lift_pair<T>(pair: (Option<T>, Option<T>)) -> Option<(T, T)> {
    let (first, second) = pair;

    first.and_then(|v1| second.and_then(|v2| Some((v1, v2))))
}
