use std::{cmp::max, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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
    pub fn insert(&mut self, val: i32) {
        if val == self.val {
            return;
        }
        if val > self.val {
            match &mut self.right {
                Some(right) => right.insert(val),
                None => self.right = Some(Box::new(TreeNode::new(val))),
            }
        } else {
            match &mut self.left {
                Some(left) => left.insert(val),
                None => self.left = Some(Box::new(TreeNode::new(val))),
            }
        }
    }

    pub fn depth(&self) -> i32 {
        let ldepth = match &self.left {
            None => 0,
            Some(left) => left.depth(),
        };
        let rdepth = match &self.right {
            None => 0,
            Some(right) => right.depth(),
        };
        let max_depth = max(ldepth, rdepth);
        return max_depth + 1;
    }

    pub fn invert(&mut self) {
        if let Some(left) = &mut self.left {
            left.invert();
        }
        if let Some(right) = &mut self.right {
            right.invert();
        }
        std::mem::swap(&mut self.left, &mut self.right);
    }
}

pub fn invert_tree(mut root: TreeNode) -> TreeNode {
    root.invert();
    root
}
fn main() {
    let mut root = TreeNode::new(45);
    root.insert(32);
    root.insert(50);
    root.insert(5);
    root.insert(40);
    root.insert(100);
    root.insert(37);
    root.insert(43);
    dbg!(&root);
    let root = root.invert();
    dbg!(root);
}
