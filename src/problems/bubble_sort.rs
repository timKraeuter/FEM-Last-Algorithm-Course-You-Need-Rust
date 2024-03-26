#[allow(dead_code)]
/**
 * Sort the array in place and then return it.
 */
fn bubble_sort(array: Vec<i32>) -> Vec<i32> {
    array
}

#[test]
fn bubble_sort_test() {
    let empty: Vec<i32> = vec![];
    assert_eq!(empty, bubble_sort(vec![]));
    assert_eq!(vec![1], bubble_sort(vec![1]));
    
    assert_eq!(vec![1, 3, 2], bubble_sort(vec![1, 3, 2]));
    assert_eq!(vec![1, 2, 5, 6, 55], bubble_sort(vec![1, 55, 3, 6, 5]));
}
