#[allow(dead_code)]
/**
 * Sort the array in place and then return it.
 * Runtime is O(n^2).
 */
fn bubble_sort(mut array: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < array.len() { // There are no old school for loops in Rust, so it is a bit verbose.
        let mut j = 0;
        while j < array.len() - 1 - i {
            if array[j] > array[j + 1] {
                // Swap manually
                // let tmp = array[j];
                // array[j] = array[j + 1];
                // array[j + 1] = tmp;

                // Swap function exists in rust.
                array.swap(j, j + 1);
            }
            j += 1;
        }
        i += 1;
    }
    array
}

#[test]
fn bubble_sort_test() {
    let empty: Vec<i32> = vec![];
    assert_eq!(empty, bubble_sort(vec![]));
    assert_eq!(vec![1], bubble_sort(vec![1]));

    assert_eq!(vec![1, 2, 3], bubble_sort(vec![1, 3, 2]));
    assert_eq!(vec![1, 3, 5, 6, 55], bubble_sort(vec![1, 55, 3, 6, 5]));
}
