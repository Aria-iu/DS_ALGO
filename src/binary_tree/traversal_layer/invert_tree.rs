//* 递归 */
pub fn invert_tree1(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root.as_ref() {
        let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());
        node.borrow_mut().left = invert_tree(right);
        node.borrow_mut().right = invert_tree(left);
    }
    root
}
//* 迭代 */
pub fn invert_tree2(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
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
