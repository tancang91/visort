use super::{add_snapshot, Sorter};

pub struct BubbleSorter;

impl<T> Sorter<T> for BubbleSorter
where
    T: Ord + Clone,
{
    fn sort(&self, slice: &[T]) -> Vec<Vec<u32>> {
        let mut swapped = true;
        let mut new_slice = slice.to_vec();

        let first: Vec<u32> = new_slice
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();

        let mut s = vec![first];

        while swapped {
            swapped = false;
            for i in 1..new_slice.len() {
                if new_slice[i - 1] > new_slice[i] {
                    new_slice.swap(i - 1, i);
                    swapped = true;

                    add_snapshot(&mut s, i - 1, i)
                }
            }
        }
        s
    }
}
