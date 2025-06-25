use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode<i32>>>>,
    root2: Option<Rc<RefCell<TreeNode<i32>>>>,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if root1.is_none() {
        return root2;
    }
    if root2.is_none() {
        return root1;
    }
    let binding = root1.clone();
    let mut node1 = binding.as_ref().unwrap().borrow_mut();
    let node2 = root2.as_ref().unwrap().borrow_mut();
    node1.left = merge_trees(node1.left.clone(), node2.left.clone());
    node1.right = merge_trees(node1.right.clone(), node2.right.clone());
    node1.val += node2.val;

    root1
}
