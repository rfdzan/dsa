pub mod binary_tree;
#[derive(Debug)]
pub struct BinaryTreeNode {
    pub data: i32,
    pub left: Option<Box<BinaryTreeNode>>,
    pub right: Option<Box<BinaryTreeNode>>,
}