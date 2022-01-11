mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;
mod gnome_sort;

pub use bubble_sort::bubble_sort;

#[allow(unused)]
pub fn is_sorted<T: Ord>(ve2: &[T]) -> bool {
    let mut sorted = true;
    for i in 0..ve2.len() - 1 {
        if ve2[i] > ve2[i + 1] {
            sorted = false
        }
    }
    sorted
}
