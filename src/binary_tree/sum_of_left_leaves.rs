use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> i32 {
    let mut res = 0;
    if let Some(node) = root {
        if let Some(left) = &node.borrow().left {
            if left.borrow().right.is_none() && left.borrow().right.is_none() {
                res += left.borrow().val;
            }
        }
        res + sum_of_left_leaves(node.borrow().left.clone())
            + sum_of_left_leaves(node.borrow().right.clone())
    } else {
        0
    }
}
