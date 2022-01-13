use std::cmp::Ordering;

#[allow(unused)]
pub fn binary_search<T: Ord>(item: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        match item.cmp(&arr[mid]) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings() {
        let index = binary_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_ints() {
        let index = binary_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
