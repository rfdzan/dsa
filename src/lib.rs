use std::{fmt::Debug, rc::Rc};

pub mod bfs;
pub mod binary_tree;

#[derive(Debug)]
pub struct BinaryTreeNode {
    pub data: i32,
    pub left: Option<Rc<BinaryTreeNode>>,
    pub right: Option<Rc<BinaryTreeNode>>,
}
