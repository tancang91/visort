mod bubblesort;

pub use bubblesort::BubbleSorter;

pub trait Sorter<T> {
    fn sort(&self, slices: &mut [T])
    where
        T: Ord;
}
