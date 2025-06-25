use super::super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut res = 0;
    if root.is_some() {
        queue.push_back(root);
    }
    while !queue.is_empty() {
        res += 1;
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap().unwrap();
            if node.borrow().left.is_some() {
                queue.push_back(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                queue.push_back(node.borrow().right.clone());
            }
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return res;
            }
        }
    }
    res
}
