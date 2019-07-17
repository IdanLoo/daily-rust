/**
 * 3. Longest Substring Without Repeating Characters
 * URL: https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-7-17
 */
pub struct Solution {}

use std::collections::HashMap;

struct Window {
  index: usize,
  length: usize,
  target: String,
}

fn has_repeat_letters(words: &str) -> bool {
  let mut map = HashMap::new();
  let mut result = false;

  words.chars().for_each(|c| {
    if map.get(&c).is_some() {
      result = true;
    } else {
      map.insert(c.clone(), true);
    }
  });

  result
}

impl Window {
  fn new(target: String) -> Window {
    Window {
      index: 0,
      length: 1,
      target: target,
    }
  }

  fn move_forward(&mut self) -> bool {
    self.index += 1;
    let next_index = self.index + self.length;

    if let Some(frame) = &self.target.get(self.index..next_index) {
      if has_repeat_letters(frame) {
        return self.move_forward();
      } else {
        return true;
      }
    }
    false
  }

  fn look_forward(&mut self) {
    let next_index = self.index + self.length;
    let frame = &self.target[self.index..next_index];

    if let Some(next) = self.target.chars().nth(next_index) {
      if frame.contains(next) {
        if !self.move_forward() {
          return;
        }
      } else {
        self.length += 1;
      }
      self.look_forward();
    }
  }
}

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    if s.is_empty() {
      return 0;
    }

    let mut window = Window::new(s);
    window.look_forward();
    window.length as i32
  }
}

#[test]
fn test() {
  let s1 = String::from("abcabcbb");
  assert_eq!(Solution::length_of_longest_substring(s1), 3);

  let s2 = String::from("bbbbb");
  assert_eq!(Solution::length_of_longest_substring(s2), 1);

  let s3 = String::from("pwwkew");
  assert_eq!(Solution::length_of_longest_substring(s3), 3);

  let s4 = String::from("");
  assert_eq!(Solution::length_of_longest_substring(s4), 0);

  let s5 = String::from("uqinntq");
  assert_eq!(Solution::length_of_longest_substring(s5), 4);
}
