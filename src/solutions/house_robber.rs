/**
 * 198. House Robber
 * URL: https://leetcode-cn.com/problems/house-robber/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-8-12
 */
pub struct Solution {}

use std::cmp::max;
use std::collections::HashMap;

struct Robber<'a> {
  book: HashMap<usize, i32>,
  nums: &'a Vec<i32>,
}

impl<'a> Robber<'a> {
  fn new(nums: &'a Vec<i32>) -> Self {
    Self {
      book: HashMap::new(),
      nums,
    }
  }

  fn max_income_behind(&mut self, index: i32) -> i32 {
    if index < 0 {
      return 0;
    }

    if index == 0 {
      return *self.nums.get(0).unwrap_or(&0);
    }

    let index = index as usize;
    if let Some(val) = self.book.get(&index) {
      return *val;
    }

    let val = max(
      self.max_income_behind(index as i32 - 2) + self.nums[index],
      self.max_income_behind(index as i32 - 1),
    );
    self.book.insert(index, val);
    val
  }
}

impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    let mut robber = Robber::new(&nums);
    robber.max_income_behind(nums.len() as i32 - 1)
  }
}

#[cfg(test)]
mod rob {
  use super::*;

  #[test]
  fn test_case() {
    let houses = vec![1, 2, 3, 1];
    let result = Solution::rob(houses);
    assert_eq!(result, 4);

    let houses = vec![2, 7, 9, 3, 1];
    let result = Solution::rob(houses);
    assert_eq!(result, 12);

    let houses = Vec::new();
    let result = Solution::rob(houses);
    assert_eq!(result, 0);

    let houses = vec![1, 1];
    let result = Solution::rob(houses);
    assert_eq!(result, 1);
  }
  #[test]
  fn only_one_house_sr_amount_of_the_house() {
    let houses = vec![10];
    let result = Solution::rob(houses);
    assert_eq!(result, 10);
  }
}
