use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

// 中序遍历
pub fn is_valid_bst1(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    let mut vec = vec![];
    valid_bst1(root, &mut vec);
    for i in 1..vec.len() {
        if vec[i] <= vec[i - 1] {
            return false;
        }
    }
    true
}

fn valid_bst1(root: Option<Rc<RefCell<TreeNode<i32>>>>, mut v: &mut Vec<i64>) {
    if root.is_none() {
        return;
    }
    let node = root.as_ref().unwrap().borrow();
    valid_bst1(node.left.clone(), v);
    v.push(node.val as i64);
    valid_bst1(node.right.clone(), v);
}

// 递归
pub fn is_valid_bst2(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    valid_bst2(i64::MIN, i64::MAX, root)
}

fn valid_bst2(low: i64, upper: i64, root: Option<Rc<RefCell<TreeNode<i32>>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let root = root.as_ref().unwrap().borrow();
    if root.val as i64 <= low || root.val as i64 >= upper {
        return false;
    }
    valid_bst2(low, root.val as i64, root.left.clone())
        && valid_bst2(root.val as i64, upper, root.right.clone())
}
