use super::node::*;
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![root];
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            res.push(node.borrow().val);
            stack.push(node.borrow().right.clone());
            stack.push(node.borrow().left.clone());
        }
    }
    res
}
//中序
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    let mut node = root;

    while !stack.is_empty() || node.is_some() {
        while let Some(n) = node {
            node = n.borrow().left.clone();
            stack.push(n);
        }
        if let Some(n) = stack.pop() {
            res.push(n.borrow().val);
            node = n.borrow().right.clone();
        }
    }
    res
}
//后序
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![root];
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            res.push(node.borrow().val);
            stack.push(node.borrow().left.clone());
            stack.push(node.borrow().right.clone());
        }
    }
    res.into_iter().rev().collect()
}
