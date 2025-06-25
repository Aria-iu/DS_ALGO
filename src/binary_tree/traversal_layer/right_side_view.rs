use super::super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }
    while !queue.is_empty() {
        let len = queue.len();
        for i in 0..len {
            let node = queue.pop_front().unwrap().unwrap();
            if i == len - 1 {
                res.push(node.borrow().val);
            }
            if node.borrow().left.is_some() {
                queue.push_back(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                queue.push_back(node.borrow().right.clone());
            }
        }
    }
    res
}
