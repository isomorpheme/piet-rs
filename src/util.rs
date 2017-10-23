pub type Coords = (usize, usize);

pub fn lift_pair<T>(pair: (Option<T>, Option<T>)) -> Option<(T, T)> {
    let (first, second) = pair;

    first.and_then(|v1| second.and_then(|v2| Some((v1, v2))))
}
