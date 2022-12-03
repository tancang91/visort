use super::{add_snapshot, Sorter};

pub struct HeapSorter;

impl<T> Sorter<T> for HeapSorter
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
        heap_sort(&mut new_slice, &mut s);
        s
    }
}

fn heap_sort<T: Ord>(arr: &mut [T], snapshot: &mut Vec<Vec<u32>>) {
    if arr.len() <= 1 {
        return;
    }

    heapify(arr, snapshot);

    for end in (1..arr.len()).rev() {
        arr.swap(0, end);
        add_snapshot(snapshot, 0, end);
        move_down(&mut arr[..end], 0, snapshot);
    }
}

fn heapify<T: Ord>(arr: &mut [T], snapshot: &mut Vec<Vec<u32>>) {
    let last_parent = (arr.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(arr, i, snapshot);
    }
}

fn move_down<T: Ord>(arr: &mut [T], mut root: usize, snapshot: &mut Vec<Vec<u32>>) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let max = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[max] > arr[root] {
            arr.swap(root, max);
            add_snapshot(snapshot, root, max);
        }
        root = max;
    }
}
