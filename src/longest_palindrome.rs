/**
 * 5. Longest Palindromic Substring
 * URL: https://leetcode-cn.com/problems/longest-palindromic-substring/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-7-19
 */
pub struct Solution {}

use std::cmp::min;

fn wrapped_string(s: &str) -> String {
  s.chars()
    .fold(String::from("$#"), |acc, c| format!("{}{}#", acc, c))
}

impl Solution {
  pub fn longest_palindrome(s: String) -> String {
    if s.len() < 2 {
      return s;
    }

    let wrapped = wrapped_string(&s);

    let ma = &mut wrapped.chars();
    let mut mp = Vec::with_capacity(wrapped.len());
    let mut mx = 0;
    let mut id = 0;

    for i in 0..wrapped.len() {
      mp[i] = if mx > i {
        min(mp[2 * id - 1], mx - i)
      } else {
        1
      };
      while ma.nth(i + mp[i]).unwrap() == ma.nth(i - mp[i]).unwrap() {
        mp[i] += 1;
      }

      if i + mp[i] > mx {
        mx = i + mp[i];
        id = 1;
      }
    }

    println!("{}, {}", mx, id);

    String::new()
  }
}

#[test]
fn short_string() {
  let result = Solution::longest_palindrome(String::from(""));
  assert_eq!(result, "");

  let result = Solution::longest_palindrome(String::from("a"));
  assert_eq!(result, "a");
}

#[test]
fn only_one_palindrome() {
  let s = String::from("babad");
  let result = Solution::longest_palindrome(s);
  assert_eq!(result, "bab");

  let s = String::from("cbbd");
  let result = Solution::longest_palindrome(s);
  assert_eq!(result, "bb");
}
