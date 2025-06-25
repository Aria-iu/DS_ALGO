use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<String> {
    let mut res = vec![];
    recur(&root, String::from(""), &mut res);
    res
}
fn recur(node: &Option<Rc<RefCell<TreeNode<i32>>>>, mut path: String, res: &mut Vec<String>) {
    let r = node.as_ref().unwrap().borrow();
    path += format!("{}", r.val).as_str();
    if r.left.is_none() && r.right.is_none() {
        res.push(path.to_string());
        return;
    }
    if r.left.is_some() {
        recur(&r.left, path.clone() + "->", res);
    }
    if r.right.is_some() {
        recur(&r.right, path + "->", res);
    }
}
