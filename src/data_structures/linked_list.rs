#[allow(unused)]
#[derive(Debug)]
struct Node {
    value: i64,
    next: Option<Box<Node>>,
}

impl Node {
    // Probably we have to use unsafe rust.

    // Insertion
    // Deletion
}

#[test]
fn linked_list_test() {
    let node = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: None,
        })),
    };
    println!("{:?}", node);
}
