use super::node::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::i32;
use std::rc::Rc;

pub fn find_mode(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut count = 0;
    let mut max_count = 0;
    let mut res = vec![];
    let mut pre = i32::MIN;
    search_bst(&root, &mut pre, &mut res, &mut count, &mut max_count);
    res
}

fn search_bst(
    cur: &Option<Rc<RefCell<TreeNode<i32>>>>,
    mut pre: &mut i32,
    res: &mut Vec<i32>,
    count: &mut i32,
    max_count: &mut i32,
) {
    if cur.is_none() {
        return;
    }

    let cur_node = cur.as_ref().unwrap().borrow();
    search_bst(&cur_node.left, pre, res, count, max_count);
    if *pre == i32::MIN {
        *count = 1;
    } else if *pre == cur_node.val {
        *count += 1;
    } else {
        *count = 1;
    };
    match count.cmp(&max_count) {
        std::cmp::Ordering::Equal => res.push(cur_node.val),
        std::cmp::Ordering::Greater => {
            *max_count = *count;
            res.clear();
            res.push(cur_node.val);
        }
        _ => {}
    };
    *pre = cur_node.val;
    search_bst(&cur_node.right, pre, res, count, max_count);
}
