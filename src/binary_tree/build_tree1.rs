use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn build_tree1(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if inorder.is_empty() {
        return None;
    }
    let mut postorder = postorder;
    let root = postorder.pop().unwrap();
    let index = inorder.iter().position(|&x| x == root).unwrap();
    let mut root = TreeNode::new(root);
    root.left = build_tree1(inorder[..index].to_vec(), postorder[..index].to_vec());
    root.right = build_tree1(inorder[index + 1..].to_vec(), postorder[index..].to_vec());
    Some(Rc::new(RefCell::new(root)))
}
