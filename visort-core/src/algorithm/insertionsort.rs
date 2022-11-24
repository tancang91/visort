use super::Sorter;

pub struct InsertionSorter;

impl<T> Sorter<T> for InsertionSorter
where
    T: Ord + Clone,
{
    fn sort(&self, slice: &[T]) -> Vec<Vec<u32>> {
        let mut new_slice = slice.to_vec();
        let first: Vec<u32> = new_slice
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();
        let mut s = vec![first];

        // [ sorted | not sorted ]
        for unsorted in 1..new_slice.len() {
            let mut i = unsorted;
            while i > 0 &&  new_slice[i - 1] > new_slice[i] {
                new_slice.swap(i - 1, i);

                let mut new = s.get(s.len() - 1).unwrap().clone();
                new.swap(i - 1, i);
                s.push(new);

                i -= 1;
            }
        }
        s
    }
}
