use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn delete_node(
    root: Option<Rc<RefCell<TreeNode<i32>>>>,
    key: i32,
) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    root.as_ref()?;

    let mut node = root.as_ref().unwrap().borrow_mut();
    match node.val.cmp(&key) {
        std::cmp::Ordering::Less => node.right = delete_node(node.right.clone(), key),
        std::cmp::Ordering::Equal => match (node.left.clone(), node.right.clone()) {
            (None, None) => return None,
            (None, Some(r)) => return Some(r),
            (Some(l), None) => return Some(l),
            (Some(l), Some(r)) => {
                let mut cur = Some(r.clone());
                while let Some(n) = cur.clone().unwrap().borrow().left.clone() {
                    cur = Some(n);
                }
                cur.unwrap().borrow_mut().left = Some(l);
                // drop(node);
                return Some(r);
            }
        },
        std::cmp::Ordering::Greater => node.left = delete_node(node.left.clone(), key),
    }
    drop(node);
    root
}
