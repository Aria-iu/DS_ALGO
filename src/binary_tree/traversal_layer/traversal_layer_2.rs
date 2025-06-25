use super::super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order_2(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut queue = VecDeque::new();
    if root.is_some() {
        queue.push_back(root);
    }
    while !queue.is_empty() {
        let mut temp = vec![];
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap().unwrap();
            temp.push(node.borrow().val);
            if node.borrow().left.is_some() {
                queue.push_back(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                queue.push_back(node.borrow().right.clone());
            }
        }
        res.push(temp);
    }
    res.into_iter().rev().collect()
}
