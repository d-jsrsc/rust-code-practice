mod searchs;
mod sort;
mod strs;

fn main() {
    let mut vec1 = [9, 32, 13, 2, 41, 33, 3, 5, 6];
    sort::bubble_sort(&mut vec1);
    //
    println!("hello")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
