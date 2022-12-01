use super::Sorter;

pub struct SelectionSorter;

impl<T> Sorter<T> for SelectionSorter
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

        for unsorted in 0..new_slice.len() {
            let smallest_in_rest = new_slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice is non-empty");

            if unsorted != smallest_in_rest {
                new_slice.swap(unsorted, smallest_in_rest);

                let mut new = s.get(s.len() - 1).unwrap().clone();
                new.swap(unsorted, smallest_in_rest);
                s.push(new);
            }
        }
        s
    }
}
