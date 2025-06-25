use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
    p: Option<Rc<RefCell<TreeNode<i32>>>>,
    q: Option<Rc<RefCell<TreeNode<i32>>>>,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if root.is_none() {
        return root;
    }
    if Rc::ptr_eq(root.as_ref().unwrap(), p.as_ref().unwrap())
        || Rc::ptr_eq(root.as_ref().unwrap(), q.as_ref().unwrap())
    {
        return root;
    }
    let left = lowest_common_ancestor(
        root.as_ref().unwrap().borrow().left.clone(),
        p.clone(),
        q.clone(),
    );
    let right = lowest_common_ancestor(root.as_ref().unwrap().borrow().right.clone(), p, q);
    match (&left, &right) {
        (None, Some(_)) => right,
        (Some(_), Some(_)) => root,
        _ => left,
    }
}
