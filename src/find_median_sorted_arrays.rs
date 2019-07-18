/**
 * 4. Median of Two Sorted Arrays
 * URL: https://leetcode-cn.com/problems/median-of-two-sorted-arrays/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-7-19
 */
pub struct Solution {}

fn find(
  num1: &Vec<i32>,
  num2: &Vec<i32>,
  current: &i32,
  current_index: &usize,
  target_index: &mut usize,
  left_part: &mut usize,
  right_part: &mut usize,
) -> Option<f64> {
  let target = &num1[*target_index];

  if left_part == right_part {
    let result = if target < current { target } else { current };
    return Some(result.clone() as f64);
  }

  if *left_part == *right_part - 1 {
    let (a, b) = if current < target {
      if let Some(next) = num2.get(*current_index + 1) {
        let min = if next < target { next } else { target };
        (current, min)
      } else {
        (current, target)
      }
    } else {
      if let Some(next) = num1.get(*target_index + 1) {
        let min = if next < current { next } else { current };
        (target, min)
      } else {
        (current, target)
      }
    };

    let result = (a + b) as f64 / 2f64;
    return Some(result);
  }

  if current < target {
    *left_part += 1;
  } else {
    *left_part += 1;
    *right_part -= 1;
    *target_index += 1;
    return find(
      num1,
      num2,
      current,
      current_index,
      target_index,
      left_part,
      right_part,
    );
  }

  None
}

impl Solution {
  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (nums1, nums2) = if nums1.len() > nums2.len() {
      (nums1, nums2)
    } else {
      (nums2, nums1)
    };

    let sum = nums1.len() + nums2.len();
    let mut target_index = 0;
    let mut left_part = 0;
    let mut right_part = sum;
    for (i, n) in nums2.iter().enumerate() {
      right_part -= 1;

      if let Some(result) = find(
        &nums1,
        &nums2,
        &n,
        &i,
        &mut target_index,
        &mut left_part,
        &mut right_part,
      ) {
        return result;
      }
    }

    let median_index = sum / 2 - nums2.len();
    if sum % 2 == 0 {
      (nums1[median_index - 1] + nums1[median_index]) as f64 / 2f64
    } else {
      nums1[median_index] as f64
    }
  }
}

#[test]
fn with_an_empty_array() {
  let nums_1 = Vec::new();
  let nums_2 = vec![1, 2, 3];

  let result = Solution::find_median_sorted_arrays(nums_1.clone(), nums_2.clone());
  assert_eq!(result, 2f64);

  let result = Solution::find_median_sorted_arrays(nums_2, nums_1);
  assert_eq!(result, 2f64);
}

#[test]
fn sum_of_arrays_counts_is_an_odd() {
  let nums_1 = vec![3, 5, 9];
  let nums_2 = vec![2, 4];

  let result = Solution::find_median_sorted_arrays(nums_1.clone(), nums_2.clone());
  assert_eq!(result, 4f64);

  let result = Solution::find_median_sorted_arrays(nums_2, nums_1);
  assert_eq!(result, 4f64);
}

#[test]
fn sum_of_arrays_counts_is_an_even() {
  let nums_1 = vec![3, 5, 9];
  let nums_2 = vec![2, 4, 8];

  let result = Solution::find_median_sorted_arrays(nums_1.clone(), nums_2.clone());
  assert_eq!(result, 4.5);

  let result = Solution::find_median_sorted_arrays(nums_2, nums_1);
  assert_eq!(result, 4.5);
}

#[test]
fn count_of_num2_is_not_enough() {
  let nums_1 = vec![5, 7, 9];
  let nums_2 = vec![3];

  let result = Solution::find_median_sorted_arrays(nums_1.clone(), nums_2.clone());
  assert_eq!(result, 6f64);

  let result = Solution::find_median_sorted_arrays(nums_2, nums_1);
  assert_eq!(result, 6f64);
}

#[test]
fn given_test_cases() {
  let nums_1 = vec![1, 2];
  let nums_2 = vec![-1, 3];
  let result = Solution::find_median_sorted_arrays(nums_1, nums_2);
  assert_eq!(result, 1.5f64);

  let nums_1 = vec![1, 2];
  let nums_2 = vec![3, 4];
  let result = Solution::find_median_sorted_arrays(nums_1, nums_2);
  assert_eq!(result, 2.5f64);
}
