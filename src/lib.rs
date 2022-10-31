mod algorithm;

pub use algorithm::{Sorter, BubbleSorter};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        let mut things = vec![4, 2, 3, 5, 1];
        BubbleSorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4, 5]);
    }
}
