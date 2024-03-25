#[allow(dead_code)]
/**
 *
 * O(sqrt(n)) time complexity.
 */
fn two_crystal_balls(breaks: Vec<bool>) -> usize {
    let jmp_amount = floor_sqrt(breaks.len());
    let mut position = jmp_amount;
    while position < breaks.len() {
        let broken = *breaks.get(position).unwrap(); // Always exists due to while condition.
        if broken {
            // First ball broken!
            break;
        } else {
            position += jmp_amount;
        }
    }
    // Only one ball left: Linear search from position - jmp to position where it broke.
    let mut last_good_position = position - jmp_amount;
    while last_good_position <= position {
        let broken = *breaks.get(last_good_position).unwrap();
        if broken {
            return last_good_position;
        }
        last_good_position += 1;
    }
    panic!("Malformed input!");
}

fn floor_sqrt(i: usize) -> usize {
    // Hide these details. SQRT for integers (isqrt()) is still unstable
    (i as f64).sqrt() as usize // as usize floors the value
}

#[test]
fn linear_search_test() {
    assert_eq!(0, two_crystal_balls(vec![true, true, true]));
    assert_eq!(1, two_crystal_balls(vec![false, true, true]));
    assert_eq!(2, two_crystal_balls(vec![false, false, true]));
    assert_eq!(
        3,
        two_crystal_balls(vec![false, false, false, true, true, true])
    );
    assert_eq!(
        14,
        two_crystal_balls(vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, true, true, true
        ])
    );
}
