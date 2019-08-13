/**
 * 1028. Recover a Tree From Preorder Traversal
 * URL: https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-8-13
 */
Definition for a binary tree node.
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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
  pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
      
  }
}