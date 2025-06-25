use super::super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

//* 递归 */
pub fn invert_tree1(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if let Some(node) = root.as_ref() {
        let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
        node.borrow_mut().left = invert_tree1(right);
        node.borrow_mut().right = invert_tree1(left);
    }
    root
}
//* 迭代 */
pub fn invert_tree2(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    let mut stack = vec![root.clone()];
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
            stack.push(right.clone());
            stack.push(left.clone());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }
    }
    root
}
