mod bubblesort;

pub use bubblesort::BubbleSorter;

pub trait Sorter<T> {
    fn sort(&self, slices: &[T]) -> Vec<Vec<u32>>
    where
        T: Ord + Clone + Copy;
}
