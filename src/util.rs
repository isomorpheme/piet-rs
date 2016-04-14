pub type Coords = (usize, usize);

pub fn lift_tuple<T>(tuple: (Option<T>, Option<T>)) -> Option<(T, T)> {
    let (first, second) = tuple;

    first.and_then(|v1| second.and_then(|v2| Some((v1, v2))))
}
