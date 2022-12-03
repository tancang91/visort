mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSorter;
pub use insertionsort::InsertionSorter;
pub use quicksort::QuickSorter;
pub use selectionsort::SelectionSorter;

pub trait Sorter<T> {
    fn sort(&self, slices: &[T]) -> Vec<Vec<u32>>
    where
        T: Ord + Clone + Copy;
}

fn add_snapshot(s: &mut Vec<Vec<u32>>, i: usize, j: usize) {
    let mut new = s.get(s.len() - 1).unwrap().clone();
    new.swap(i, j);
    s.push(new);
}
