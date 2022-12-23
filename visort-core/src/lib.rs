mod algorithm;

pub use algorithm::{
    BubbleSorter, HeapSorter, InsertionSorter, MergeSorter, QuickSorter, SelectionSorter, Sorter,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let mut things = vec![4, 2, 13, 50, 10];
        let res = MergeSorter.sort(&mut things);
        for i in &res {
            eprintln!("{:?}", i);
        }
        assert_eq!(res[res.len() - 1], vec![1, 0, 4, 2, 3]);
    }
}
