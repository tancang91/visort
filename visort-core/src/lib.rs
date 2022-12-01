mod algorithm;

pub use algorithm::{BubbleSorter, InsertionSorter, Sorter};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let mut things = vec![4, 2, 13, 50, 10];
        let res = BubbleSorter.sort(&mut things);
        for i in &res {
            eprintln!("{:?}", i);
        }
        //assert_eq!(res[res.len() - 1], vec![1, 2, 3, 4, 5]);
    }
}
