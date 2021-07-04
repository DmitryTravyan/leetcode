use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution();

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::<i32>::new();
        if let Some(tr) = root {
            match Rc::try_unwrap(tr).unwrap().into_inner() {
                TreeNode {
                    val,
                    left,
                    right
                } if left.is_some() && right.is_none() => {
                    v.extend_from_slice(&*Solution::inorder_traversal(left));
                    v.push(val);
                }
                TreeNode {
                    val,
                    left,
                    right
                } if left.is_none() && right.is_some() => {
                    v.push(val);
                    v.extend_from_slice(&*Solution::inorder_traversal(right));
                }
                TreeNode {
                    val,
                    left,
                    right
                } if left.is_some() && right.is_some() => {
                    v.extend_from_slice(&*Solution::inorder_traversal(left));
                    v.push(val);
                    v.extend_from_slice(&*Solution::inorder_traversal(right));
                }
                TreeNode { val, .. } => {
                    v.push(val);
                }
            }
        }
        v
    }
}

pub fn main() {
}