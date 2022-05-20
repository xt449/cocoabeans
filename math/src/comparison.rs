use std::cmp::Ordering;

pub fn min_max<T: Ord>(v1: T, v2: T) -> (T, T) {
    match Ord::cmp(&v1, &v2) {
        Ordering::Less | Ordering::Equal => (v1, v2),
        Ordering::Greater => (v2, v1),
    }
}
