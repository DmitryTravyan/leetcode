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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::<i32>::new();
        if let Some(tr) = root {
            let x = Rc::try_unwrap(tr).unwrap().into_inner();
            match x {
                TreeNode {
                    val,
                    left,
                    right
                } => {
                    v.extend_from_slice(&*Solution::postorder_traversal(left));
                    v.extend_from_slice(&*Solution::postorder_traversal(right));
                    v.push(val);
                }
            }
        }
        v
    }
}

fn main() {
    println!("Hello, world!");
}
