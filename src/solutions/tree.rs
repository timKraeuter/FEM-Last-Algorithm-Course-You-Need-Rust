#[allow(unused)]
pub fn make_test_tree() -> BTree {
    BTree::node(
        20,
        BTree::node(10, BTree::no_left(5, BTree::leaf(7)), BTree::leaf(15)),
        BTree::node(
            50,
            BTree::node(30, BTree::leaf(29), BTree::leaf(45)),
            BTree::leaf(100),
        ),
    )
}

pub struct BTree {
    pub value: i32,
    pub left: Option<Box<BTree>>,
    pub right: Option<Box<BTree>>,
}

impl BTree {
    fn node(value: i32, left: BTree, right: BTree) -> BTree {
        BTree {
            value,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn leaf(value: i32) -> BTree {
        BTree {
            value,
            left: None,
            right: None,
        }
    }

    fn no_left(value: i32, right: BTree) -> BTree {
        BTree {
            value,
            left: None,
            right: Some(Box::new(right)),
        }
    }
}