#[allow(dead_code)]
/**
 * Sort the array in place and then return it.
 * Runtime complexity O(n^2) (worst case).
 */
fn quick_sort(mut array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        return array;
    }
    let high = array.len() - 1;
    qs(&mut array, 0, high); // sort in place.
    array
}

fn qs(arr: &mut Vec<i32>, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }
    let pivot_idx = partition(arr, lo, hi);
    qs(arr, lo, pivot_idx - 1);
    qs(arr, pivot_idx + 1, hi);
}

fn partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi]; // Pivot is chosen to be last but could also be middle.
    let mut idx = lo as i32 - 1; // let it become negative up to -1

    let mut i = lo;
    while i < hi {
        if arr[i] <= pivot {
            idx += 1;
            arr.swap(i, idx as usize); // idx is 0 or higher, so it can be cast to usize
        }
        i += 1;
    }
    idx += 1;
    let idx = idx as usize; // idx is 0 or higher, so it can be cast to usize

    arr.swap(idx, hi);
    idx
}

#[test]
fn quick_sort_test() {
    let empty: Vec<i32> = vec![];
    assert_eq!(empty, quick_sort(vec![]));
    assert_eq!(vec![1], quick_sort(vec![1]));

    assert_eq!(vec![1, 2, 3], quick_sort(vec![1, 3, 2]));
    assert_eq!(vec![1, 3, 5, 6, 55], quick_sort(vec![1, 55, 3, 6, 5]));
    assert_eq!(
        vec![3, 4, 7, 9, 42, 69, 420],
        quick_sort(vec![9, 3, 7, 4, 69, 420, 42])
    );
}
