use super::Sorter;

pub struct BubbleSorter;

impl<T> Sorter<T> for BubbleSorter
where
    T: Ord,
{
    fn sort(&self, slice: &mut [T]) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}
