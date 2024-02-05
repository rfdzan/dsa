use std::rc::Rc;

use crate::BinaryTreeNode;
pub fn main_binary_tree() {
    let nodeleft3 = BinaryTreeNode {
        data: 15,
        left: None,
        right: None,
    };
    let noderight3 = BinaryTreeNode {
        data: 80,
        left: None,
        right: None,
    };
    let nodeleft2 = BinaryTreeNode {
        data: 30,
        left: Some(Rc::new(nodeleft3)),
        right: None,
    };
    let noderight2 = BinaryTreeNode {
        data: 60,
        left: None,
        right: Some(Rc::new(noderight3)),
    };
    let node1 = BinaryTreeNode {
        data: 56,
        left: Some(Rc::new(nodeleft2)),
        right: Some(Rc::new(noderight2)),
    };
    search_binary_tree(30, Rc::new(node1));
}
fn search_binary_tree(val: i32, root: Rc<BinaryTreeNode>) {
    if root.data == val {
        dbg!(&root);
    } else if val > root.data {
        match &root.right {
            Some(rc_root) => {
                let new_root = Rc::clone(rc_root);
                search_binary_tree(val, new_root)
            }
            None => println!("{val} is not found anywhere in this tree"),
        }
    } else {
        match &root.left {
            Some(rc_root) => {
                let new_root = Rc::clone(rc_root);
                search_binary_tree(val, new_root)
            }
            None => println!("{val} is not found anywhere in this tree"),
        }
    }
}
