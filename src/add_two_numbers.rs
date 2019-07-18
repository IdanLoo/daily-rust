/**
 * 2. Add Two Numbers
 * URL: https://leetcode-cn.com/problems/two-sum/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-7-16
 */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  pub fn print_node(node: &Self) {
    if let Some(next) = &node.next {
      Self::print_node(next);
    }
    print!("{}", node.val);
  }

  pub fn print(self) {
    Self::print_node(&self);
    println!();
  }
}

pub struct Solution {}

impl Solution {
  fn option_list_none(val: i32) -> Option<Box<ListNode>> {
    Some(Box::from(ListNode::new(val)))
  }

  fn add_two_nodes(a: &ListNode, b: &ListNode, to: &mut ListNode) {
    let mut carry = 0;
    to.val = a.val + b.val + to.val;

    if to.val > 9 {
      to.val -= 10;
      carry = 1;
    }

    if None == a.next.as_ref().or(b.next.as_ref()) && carry != 1 {
      return;
    }

    to.next = Self::option_list_none(carry);
    let next_to = to.next.as_mut().unwrap().as_mut();

    let zero_a = Self::option_list_none(0);
    let zero_b = Self::option_list_none(0);
    let next_a = a.next.as_ref().or(zero_a.as_ref()).unwrap().as_ref();
    let next_b = b.next.as_ref().or(zero_b.as_ref()).unwrap().as_ref();

    Self::add_two_nodes(next_a, next_b, next_to);
  }

  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut result = ListNode::new(0);
    Self::add_two_nodes(l1.unwrap().as_ref(), l2.unwrap().as_ref(), &mut result);

    Some(Box::from(result))
  }
}

#[test]
fn test() {
  let l1 = ListNode::new(5);

  let a = Some(Box::from(l1.clone()));
  let b = Some(Box::from(l1.clone()));

  let result = Solution::add_two_numbers(a, b);
  result.unwrap().print();
}
