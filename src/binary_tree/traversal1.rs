use super::node::*;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    traverse1(&root, &mut res);
    res
}

//前序遍历
pub fn traverse1(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        res.push(node.borrow().val);
        traverse(&node.borrow().left, res);
        traverse(&node.borrow().right, res);
    }
}
//后序遍历
pub fn traverse2(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        traverse(&node.borrow().left, res);
        traverse(&node.borrow().right, res);
        res.push(node.borrow().val);
    }
}
//中序遍历
pub fn traverse3(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(node) = root {
        traverse(&node.borrow().left, res);
        res.push(node.borrow().val);
        traverse(&node.borrow().right, res);
    }
}
