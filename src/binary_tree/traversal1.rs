use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut res = vec![];
    traverse1(&root, &mut res);
    res
}

//前序遍历
pub fn traverse1(root: &Option<Rc<RefCell<TreeNode<i32>>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        res.push(node.borrow().val);
        traverse1(&node.borrow().left, res);
        traverse1(&node.borrow().right, res);
    }
}
//后序遍历
pub fn traverse2(root: &Option<Rc<RefCell<TreeNode<i32>>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        traverse2(&node.borrow().left, res);
        traverse2(&node.borrow().right, res);
        res.push(node.borrow().val);
    }
}
//中序遍历
pub fn traverse3(root: &Option<Rc<RefCell<TreeNode<i32>>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        traverse3(&node.borrow().left, res);
        res.push(node.borrow().val);
        traverse3(&node.borrow().right, res);
    }
}
