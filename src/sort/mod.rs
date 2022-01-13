mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;
mod gnome_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod odd_even_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;
mod shell_sort;
mod stooge_sort;

pub use bubble_sort::bubble_sort;

pub use quick_sort::partition;

#[allow(unused)]
pub fn is_sorted<T: Ord>(ve2: &[T]) -> bool {
    if ve2.is_empty() {
        return true;
    }
    let mut sorted = true;
    for i in 0..ve2.len() - 1 {
        if ve2[i] > ve2[i + 1] {
            sorted = false
        }
    }
    sorted
}
