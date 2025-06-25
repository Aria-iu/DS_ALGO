use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> i32 {
    let mut vec = vec![];
    traversal(root, &mut vec);
    let mut min = i32::MAX;
    for i in 1..vec.len() {
        min = min.min(vec[i] - vec[i - 1])
    }
    min
}
pub fn traversal(root: Option<Rc<RefCell<TreeNode<i32>>>>, v: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }
    let node = root.as_ref().unwrap().borrow();
    traversal(node.left.clone(), v);
    v.push(node.val);
    traversal(node.right.clone(), v);
}
