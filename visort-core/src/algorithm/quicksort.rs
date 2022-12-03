use super::{add_snapshot, Sorter};

pub struct QuickSorter;

impl<T> Sorter<T> for QuickSorter
where
    T: Ord + Clone,
{
    fn sort(&self, slice: &[T]) -> Vec<Vec<u32>> {
        let mut new_slice = slice.to_vec();
        let len = new_slice.len();

        let first: Vec<u32> = new_slice
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect();

        let mut s = vec![first];
        _quick_sort(&mut new_slice, 0, (len - 1) as isize, &mut s);
        s
    }
}

fn partition<T: PartialOrd>(
    arr: &mut [T],
    lo: isize,
    hi: isize,
    snapshot: &mut Vec<Vec<u32>>,
) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
            add_snapshot(snapshot, i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    add_snapshot(snapshot, i as usize, pivot as usize);

    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize, snapshot: &mut Vec<Vec<u32>>) {
    if lo < hi {
        let p = partition(arr, lo, hi, snapshot);
        _quick_sort(arr, lo, p - 1, snapshot);
        _quick_sort(arr, p + 1, hi, snapshot);
    }
}
