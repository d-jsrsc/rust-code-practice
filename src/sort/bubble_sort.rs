pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        let mut vec1 = [9, 32, 13, 2, 41, 33, 3, 5, 6];
        bubble_sort(&mut vec1);
        for i in 0..vec1.len() - 1 {
            assert!(vec1[i] <= vec1[i + 1]);
        }
    }

    #[test]
    fn ascending() {
        let mut vec2 = [1, 2, 3, 4, 5, 6];
        bubble_sort(&mut vec2);
        for i in 0..vec2.len() - 1 {
            assert!(vec2[i] <= vec2[i + 1]);
        }
    }
}
