mod bubblesort;
mod insertionsort;
mod selectionsort;

pub use bubblesort::BubbleSorter;
pub use insertionsort::InsertionSorter;
pub use selectionsort::SelectionSorter;

pub trait Sorter<T> {
    fn sort(&self, slices: &[T]) -> Vec<Vec<u32>>
    where
        T: Ord + Clone + Copy;
}
