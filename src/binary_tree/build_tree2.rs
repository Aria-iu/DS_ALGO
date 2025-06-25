use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn build_tree2(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if preorder.is_empty() {
        return None;
    }
    let root = preorder[0];
    let index = inorder.iter().position(|&x| x == root).unwrap();
    let mut root = TreeNode::new(root);
    root.left = build_tree2(preorder[1..index + 1].to_vec(), inorder[0..index].to_vec());
    root.right = build_tree2(
        preorder[index + 1..].to_vec(),
        inorder[index + 1..].to_vec(),
    );
    Some(Rc::new(RefCell::new(root)))
}
