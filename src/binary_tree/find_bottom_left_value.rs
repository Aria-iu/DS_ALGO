use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

//*递归*/
pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> i32 {
    let mut res = 0;
    let mut max_depth = i32::MIN;
    traversal(root, 0, &mut max_depth, &mut res);
    res
}
fn traversal(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
    depth: i32,
    max_depth: &mut i32,
    res: &mut i32,
) {
    let node = root.unwrap();
    if node.borrow().left.is_none() && node.borrow().right.is_none() {
        if depth > *max_depth {
            *max_depth = depth;
            *res = node.borrow().val;
        }
        return;
    }
    if node.borrow().left.is_some() {
        traversal(node.borrow().left.clone(), depth + 1, max_depth, res);
    }
    if node.borrow().right.is_some() {
        traversal(node.borrow().right.clone(), depth + 1, max_depth, res);
    }
}
