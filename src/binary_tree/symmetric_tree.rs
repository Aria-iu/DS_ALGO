use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

fn is_symmetric(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    recur(
        &root.as_ref().unwrap().borrow().left,
        &root.as_ref().unwrap().borrow().right,
    )
}
pub fn recur(
    left: &Option<Rc<RefCell<TreeNode<i32>>>>,
    right: &Option<Rc<RefCell<TreeNode<i32>>>>,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(n1), Some(n2)) => {
            return n1.borrow().val == n2.borrow().val
                && recur(&n1.borrow().left, &n2.borrow().right)
                && recur(&n1.borrow().right, &n2.borrow().left);
        }
        _ => false,
    }
}
