use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    // 加和
    let mut pre = 0;
    traversal(&root, &mut pre);
    root
}

fn traversal(cur: &Option<Rc<RefCell<TreeNode<i32>>>>, pre: &mut i32) {
    if cur.is_none() {
        return;
    }
    let mut node = cur.as_ref().unwrap().borrow_mut();
    traversal(&node.right, pre);
    *pre += node.val;
    node.val = *pre;
    traversal(&node.left, pre);
}
