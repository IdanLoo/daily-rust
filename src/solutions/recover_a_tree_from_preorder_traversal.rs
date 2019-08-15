/**
 * 1028. Recover a Tree From Preorder Traversal
 * URL: https://leetcode-cn.com/problems/recover-a-tree-from-preorder-traversal/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-8-13
 */
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

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
  val: i32,
  dep: usize,
}

impl Node {
  fn parse(origin: &str) -> Vec<Self> {
    let mut nodes = Vec::new();
    let mut dep = 0;
    for item in origin.split("-") {
      if item.is_empty() {
        dep += 1;
      } else {
        nodes.push(Self {
          val: item.parse().unwrap(),
          dep,
        });
        dep = 1;
      }
    }

    nodes
  }

  fn to_tree_node(&self) -> TreeNode {
    TreeNode::new(self.val)
  }
}

impl Solution {
  pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
    let nodes = Node::parse(&s);
    let root = nodes.first()?.to_tree_node();
    let mut stack = vec![(0_usize, root)];

    for node in &nodes[1..] {
      let (dep, _) = stack.last().unwrap();

      if node.dep > *dep {
        let tn = node.to_tree_node();
        stack.push((node.dep, tn));
        continue;
      }

      loop {
        let (dep, top) = stack.pop().unwrap();
        let (_, father) = stack.last_mut().unwrap();

        let new_tn = Some(Rc::new(RefCell::new(top)));
        if father.left.is_none() {
          father.left = new_tn;
        } else {
          father.right = new_tn;
        }

        if dep == node.dep {
          stack.push((dep, node.to_tree_node()));
          break;
        }
      }
    }

    while stack.len() > 1 {
      let (_, top) = stack.pop().unwrap();
      let (_, father) = stack.last_mut().unwrap();

      let new_tn = Some(Rc::new(RefCell::new(top)));
      if father.left.is_none() {
        father.left = new_tn;
      } else {
        father.right = new_tn;
      }
    }

    let (_, root) = stack.pop().unwrap();
    Some(Rc::new(RefCell::new(root)))
  }
}

#[cfg(test)]
mod parse {
  use super::*;
  #[test]
  fn a_tree_string_sr_a_set_of_nodes() {
    let origin = "1-2--4--5-3--6---7";
    let result = Node::parse(origin);
    assert_eq!(result.len(), 7);
    assert_eq!(result[0], Node { val: 1, dep: 0 });
    assert_eq!(result[1], Node { val: 2, dep: 1 });
    assert_eq!(result[2], Node { val: 4, dep: 2 });
    assert_eq!(result[3], Node { val: 5, dep: 2 });
    assert_eq!(result[4], Node { val: 3, dep: 1 });
    assert_eq!(result[5], Node { val: 6, dep: 2 });
    assert_eq!(result[6], Node { val: 7, dep: 3 });
  }
}

#[cfg(test)]
mod recover_from_preorder {
  use super::*;
  macro_rules! tree {
    ($val:expr) => {{
      let root = super::TreeNode::new($val);
      Some(std::rc::Rc::new(std::cell::RefCell::new(root)))
    }};

    ($val:expr => [ $left:expr $(, $right:expr )? ] ) => {{
      let mut root = super::TreeNode::new($val);
      root.left = $left;
      $(root.right = $right;)?
      Some(std::rc::Rc::new(std::cell::RefCell::new(root)))
    }};
  }

  #[test]
  fn test_case() {
    let tn = tree!(
      1 => [
        tree!(
          2 => [
            tree!(3),
            tree!(4 => [
              tree!(8)
            ])
          ]
        ),
        tree!(5 => [
          tree!(6),
          tree!(7)
        ])
      ]
    );
    let origin = String::from("1-2--3--4---8-5--6--7");
    let result = Solution::recover_from_preorder(origin).unwrap();
    assert_eq!(result, tn.unwrap());

    let tn = tree!(
      1 => [
        tree!(
          2 => [
            tree!(3),
            tree!(4)
          ]
        ),
        tree!(
          5 => [
            tree!(6),
            tree!(7)
          ]
        )
      ]
    );
    let origin = String::from("1-2--3--4-5--6--7");
    let result = Solution::recover_from_preorder(origin).unwrap();
    assert_eq!(result, tn.unwrap());

    let tn = tree!(
      1 => [
        tree!(
          2 => [
            tree!(
              3 => [ tree!(4) ]
            )
          ]
        ),
        tree!(
          5 => [
            tree!(
              6 => [ tree!(7) ]
            )
          ]
        )
      ]
    );
    let origin = String::from("1-2--3---4-5--6---7");
    let result = Solution::recover_from_preorder(origin).unwrap();
    assert_eq!(result, tn.unwrap());

    let tn = tree!(
      1 => [
        tree!(
          401 => [
            tree!(
              349 => [ tree!(90) ]
            ),
            tree!(88)
          ]
        )
      ]
    );
    let origin = String::from("1-401--349---90--88");
    let result = Solution::recover_from_preorder(origin).unwrap();
    assert_eq!(result, tn.unwrap());
  }

  #[test]
  fn an_empty_string_sr_a_none() {
    let origin = String::from("");
    let result = Solution::recover_from_preorder(origin);
    assert!(result.is_none());
  }

  #[test]
  fn a_single_number_sr_a_tree_with_one_node() {
    let tn = tree!(3);
    let origin = String::from("3");
    let result = Solution::recover_from_preorder(origin);
    assert_eq!(result, tn);
  }
}
