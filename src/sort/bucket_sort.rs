#[allow(unused)]

// 桶排序
pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }
    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];

    for x in arr {
        buckets[len * *x / max].push(*x);
    }

    // for bucket in buckets.iter_mut() {
    //     super
    // }
    return vec![];
}

#[cfg(test)]
mod tests {}
