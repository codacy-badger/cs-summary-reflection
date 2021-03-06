use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::pre_structs::{Solution, TreeNode};

///二叉树的深度 = leetcode 104
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn get_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(root) = root {
                let node = root.try_borrow().unwrap();
                return max(get_depth(&node.left), get_depth(&node.right)) + 1;
            } else {
                return 0;
            }
        }
        get_depth(&root)
    }
}
