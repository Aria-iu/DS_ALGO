use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode<i32>>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }
    let node = root.unwrap();
    if node.borrow().left.is_none() && node.borrow().right.is_none() {
        return node.borrow().val == target_sum;
    }
    return has_path_sum(node.borrow().left.clone(), target_sum - node.borrow().val)
        || has_path_sum(node.borrow().right.clone(), target_sum - node.borrow().val);
}
