use super::Sorter;

pub struct BubbleSorter;

impl<T> Sorter<T> for BubbleSorter
where
    T: Ord + Clone,
{
    fn sort(&self, slice: &mut [T]) -> Vec<Vec<T>> {
        let mut swapped = true;
        let mut s = vec![slice.to_vec()];

        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    s.push(slice.to_vec());
                    swapped = true;
                }
            }
        }
        s
    }
}
