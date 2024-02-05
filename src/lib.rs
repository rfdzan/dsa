use std::{rc::Rc, fmt::Debug};

pub mod binary_tree;
pub mod bfs;

#[derive(Debug)]
pub struct BinaryTreeNode {
    pub data: i32,
    pub left: Option<Rc<BinaryTreeNode>>,
    pub right: Option<Rc<BinaryTreeNode>>,
}
