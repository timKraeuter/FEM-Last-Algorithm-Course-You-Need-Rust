#[allow(dead_code)]
/**
 * Sort the array in place and then return it.
 */
fn quick_sort(mut array: Vec<i32>) -> Vec<i32> {
    array
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
