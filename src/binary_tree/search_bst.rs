use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn search_bst(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if root.is_none() || root.as_ref().unwrap().borrow().val == val {
        return root;
    }
    let node_val = root.as_ref().unwrap().borrow().val;
    if node_val > val {
        return search_bst(root.as_ref().unwrap().borrow().left.clone(), val);
    }
    if node_val < val {
        return search_bst(root.unwrap().borrow().right.clone(), val);
    }
    None
}
