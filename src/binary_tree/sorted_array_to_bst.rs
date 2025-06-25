use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if nums.is_empty() {
        return None;
    }
    let index = nums.len() / 2;
    let mut root = TreeNode::new(nums[index]);

    root.left = sorted_array_to_bst(nums[..index].to_vec());
    root.right = sorted_array_to_bst(nums[index + 1..].to_vec());
    Some(Rc::new(RefCell::new(root)))
}
