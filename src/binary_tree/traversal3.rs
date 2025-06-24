// 前序
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    if root.is_some() {
        stack.push(root);
    }
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            if node.borrow().right.is_some() {
                stack.push(node.borrow().right.clone());
            }
            if node.borrow().left.is_some() {
                stack.push(node.borrow().left.clone());
            }
            stack.push(Some(node));
            stack.push(None);
        } else {
            res.push(stack.pop().unwrap().unwrap().borrow().val);
        }
    }
    res
}
// 中序
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    if root.is_some() {
        stack.push(root);
    }
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            if node.borrow().right.is_some() {
                stack.push(node.borrow().right.clone());
            }
            stack.push(Some(node.clone()));
            stack.push(None);
            if node.borrow().left.is_some() {
                stack.push(node.borrow().left.clone());
            }
        } else {
            res.push(stack.pop().unwrap().unwrap().borrow().val);
        }
    }
    res
}
// 后序
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut stack = vec![];
    if root.is_some() {
        stack.push(root);
    }
    while !stack.is_empty() {
        if let Some(node) = stack.pop().unwrap() {
            stack.push(Some(node.clone()));
            stack.push(None);
            if node.borrow().right.is_some() {
                stack.push(node.borrow().right.clone());
            }
            if node.borrow().left.is_some() {
                stack.push(node.borrow().left.clone());
            }
        } else {
            res.push(stack.pop().unwrap().unwrap().borrow().val);
        }
    }
    res
}
