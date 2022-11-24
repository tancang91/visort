mod bubblesort;
mod insertionsort;

pub use bubblesort::BubbleSorter;
pub use insertionsort::InsertionSorter;

pub trait Sorter<T> {
    fn sort(&self, slices: &[T]) -> Vec<Vec<u32>>
    where
        T: Ord + Clone + Copy;
}
