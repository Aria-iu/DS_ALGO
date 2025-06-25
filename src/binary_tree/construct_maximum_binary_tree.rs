use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn construct_maximum_binary_tree(mut nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode<i32>>>> {
    if nums.is_empty() {
        return None;
    }
    let mut max_value_index = 0;
    for i in 0..nums.len() {
        if nums[max_value_index] < nums[i] {
            max_value_index = i;
        }
    }
    let right = construct_maximum_binary_tree(nums.split_off(max_value_index + 1));
    let root = nums.pop().unwrap();
    let left = construct_maximum_binary_tree(nums);
    Some(Rc::new(RefCell::new(TreeNode {
        val: root,
        left,
        right,
    })))
}
