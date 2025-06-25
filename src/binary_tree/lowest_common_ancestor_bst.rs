use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn lowest_common_ancestor_bst(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
    p: Option<Rc<RefCell<TreeNode<i32>>>>,
    q: Option<Rc<RefCell<TreeNode<i32>>>>,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let q_val = q.as_ref().unwrap().borrow().val;
    let p_val = p.as_ref().unwrap().borrow().val;
    let root_val = root.as_ref().unwrap().borrow().val;

    if root_val > q_val && root_val > p_val {
        return lowest_common_ancestor_bst(root.as_ref().unwrap().borrow().left.clone(), p, q);
    };

    if root_val < q_val && root_val < p_val {
        return lowest_common_ancestor_bst(root.as_ref().unwrap().borrow().right.clone(), p, q);
    }
    root
}
