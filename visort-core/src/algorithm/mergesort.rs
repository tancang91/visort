use super::Sorter;

pub struct MergeSorter;

impl<T> Sorter<T> for MergeSorter
where
    T: Ord + Clone + Copy,
{
    fn sort(&self, slice: &[T]) -> Vec<Vec<u32>> {
        let mut new_slice = slice.to_vec();

        let first: Vec<u32> = new_slice
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();

        let mut s = vec![first];
        bottom_up_merge_sort(&mut new_slice, &mut s);
        s
    }
}

fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    // Create temporary vectors to support the merge.
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // Indexes to track the positions while merging.
    let mut l = 0;
    let mut r = 0;

    for v in arr {
        // Choose either the smaller element, or from whichever vec is not exhausted.
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

pub fn bottom_up_merge_sort<T: Ord + Copy>(a: &mut [T], snapshot: &mut Vec<Vec<u32>>) {
    let ref_vec = a.to_vec();

    if a.len() > 1 {
        let len: usize = a.len();
        let mut sub_array_size: usize = 1;
        while sub_array_size < len {
            let mut start_index: usize = 0;
            // still have more than one sub-arrays to merge
            while len - start_index > sub_array_size {
                let end_idx: usize = if start_index + 2 * sub_array_size > len {
                    len
                } else {
                    start_index + 2 * sub_array_size
                };
                // merge a[start_index..start_index+sub_array_size] and a[start_index+sub_array_size..end_idx]
                // NOTE: mid is a relative index number starting from `start_index`
                merge(&mut a[start_index..end_idx], sub_array_size);
                // update `start_index` to merge the next sub-arrays
                start_index = end_idx;

                // Add snapshot
                let new_s = extract_position(&a, &ref_vec);
                let previous_s = snapshot.get(snapshot.len() - 1).unwrap();
                if !new_s.eq(previous_s) {
                    snapshot.push(new_s);
                }
            }
            sub_array_size *= 2;
        }
    }
}

fn extract_position<T: Ord + Copy>(sorted_vec: &[T], ref_vec: &Vec<T>) -> Vec<u32> {
    sorted_vec
        .iter()
        .map(|&val| ref_vec.iter().position(|&r| r == val).unwrap() as u32)
        .collect()
}
