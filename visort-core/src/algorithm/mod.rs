mod bubblesort;

pub use bubblesort::BubbleSorter;

pub trait Sorter<T> {
    fn sort(&self, slices: &mut [T]) -> Vec<Vec<T>>
    where
        T: Ord + Clone + Copy;
}
