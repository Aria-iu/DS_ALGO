use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
    low: i32,
    high: i32,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    root.as_ref()?;
    let mut node = root.as_ref().unwrap().borrow_mut();
    if node.val < low {
        return trim_bst(node.right.clone(), low, high);
    }
    if node.val > high {
        return trim_bst(node.left.clone(), low, high);
    }

    node.left = trim_bst(node.left.clone(), low, high);
    node.right = trim_bst(node.right.clone(), low, high);
    drop(node);
    root
}
