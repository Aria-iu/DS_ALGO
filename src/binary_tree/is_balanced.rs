use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    get_depth(root) != -1
}

fn get_depth(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let right = get_depth(root.as_ref().unwrap().borrow().left.clone());
    let left = get_depth(root.unwrap().borrow().right.clone());
    if right == -1 {
        return -1;
    }
    if left == -1 {
        return -1;
    }
    if (right - left).abs() > 1 {
        return -1;
    }

    1 + right.max(left)
}
