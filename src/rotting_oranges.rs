/**
 * 994. Rotting Oranges
 * URL: https://leetcode-cn.com/problems/rotting-oranges/
 * Author: Idan Loo <im@siwei.lu>
 * Date: 2019-8-7
 */
pub struct Solution {}

use std::collections::VecDeque;

struct Item {
  x: usize,
  y: usize,
}

impl Item {
  fn new(x: usize, y: usize) -> Item {
    Item { x, y }
  }

  fn rot(&self, grid: &mut Vec<Vec<i32>>) -> Vec<Item> {
    let result = vec![
      self.rot_one(grid, -1, 0),
      self.rot_one(grid, 0, 1),
      self.rot_one(grid, 1, 0),
      self.rot_one(grid, 0, -1),
    ];

    result
      .into_iter()
      .filter(|r| r.is_some())
      .map(|r| r.unwrap())
      .collect::<Vec<Item>>()
  }

  fn rot_one(&self, grid: &mut Vec<Vec<i32>>, diff_x: i32, diff_y: i32) -> Option<Item> {
    let target_x = {
      let target = self.x as i32 + diff_x;
      if target < 0 {
        return None;
      }
      target as usize
    };

    let target_y = {
      let target = self.y as i32 + diff_y;
      if target < 0 {
        return None;
      }
      target as usize
    };

    if grid.get(target_y).is_none() {
      return None;
    }

    let line = grid.get(target_y).unwrap();
    if let Some(val) = line.get(target_x) {
      if *val == 1 {
        grid[target_y][target_x] = 2;
        return Some(Item::new(target_x, target_y));
      }
    }

    None
  }
}

impl Solution {
  pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    let (has_fresh, queue) = Self::rottings_of(&grid);

    if !has_fresh {
      return 0;
    }

    let queue = VecDeque::from(queue);
    let result = Self::map_queue(queue, grid.as_mut());

    for line in grid {
      for val in line {
        if val == 1 {
          return -1;
        }
      }
    }

    result as i32 - 1
  }

  fn rottings_of(grid: &Vec<Vec<i32>>) -> (bool, Vec<Item>) {
    let mut has_fresh = false;
    let mut rottings = Vec::new();

    for (y, line) in grid.iter().enumerate() {
      for (x, val) in line.iter().enumerate() {
        if *val == 1 {
          has_fresh = true;
        } else if *val == 2 {
          rottings.push(Item::new(x, y));
        }
      }
    }

    (has_fresh, rottings)
  }

  fn map_queue(queue: VecDeque<Item>, grid: &mut Vec<Vec<i32>>) -> u32 {
    if queue.is_empty() {
      return 0;
    }

    let mut new_queue = VecDeque::new();
    for item in queue {
      for new_item in item.rot(grid) {
        new_queue.push_back(new_item);
      }
    }

    return 1 + Self::map_queue(new_queue, grid);
  }
}

#[cfg(test)]
mod rotting_of {
  use super::*;
  #[test]
  fn an_empty_grid_sr_none() {
    let grid: Vec<Vec<i32>> = Vec::new();
    let (has_fresh, rottings) = Solution::rottings_of(&grid);

    assert!(!has_fresh);
    assert!(rottings.is_empty());
  }

  #[test]
  fn a_grid_without_2_sr_none() {
    let grid = vec![vec![0, 0, 1], vec![1, 0, 0]];
    let (has_fresh, rottings) = Solution::rottings_of(&grid);

    assert!(has_fresh);
    assert!(rottings.is_empty());
  }

  #[test]
  fn a_grid_with_2s_sr_an_items_vec() {
    let grid = vec![vec![0, 1, 2], vec![0, 0, 1]];
    let (has_fresh, rottings) = Solution::rottings_of(&grid);
    assert!(has_fresh);
    assert_eq!(rottings.len(), 1);

    let item = &rottings[0];
    assert_eq!(item.x, 2);
    assert_eq!(item.y, 0);

    let grid = vec![vec![0, 1, 2], vec![0, 2, 1]];
    let (has_fresh, rottings) = Solution::rottings_of(&grid);
    assert!(has_fresh);
    assert_eq!(rottings.len(), 2);

    let item_0 = &rottings[0];
    assert_eq!(item_0.x, 2);
    assert_eq!(item_0.y, 0);

    let item_1 = &rottings[1];
    assert_eq!(item_1.x, 1);
    assert_eq!(item_1.y, 1);
  }
}

#[cfg(test)]
mod rot {
  use super::*;
  #[test]
  fn an_item_in_the_bound_sr_a_new_rotted_vec() {
    let item = Item::new(0, 0);
    let mut grid = vec![vec![2, 1, 0], vec![0, 1, 0]];

    let rotted = item.rot(grid.as_mut());
    assert_eq!(rotted.len(), 1);

    let new_item = &rotted[0];
    assert_eq!(new_item.x, 1);
    assert_eq!(new_item.y, 0);
  }

  #[test]
  fn an_item_in_centre_sr_a_new_rotted_vec() {
    let item = Item::new(1, 1);
    let mut grid = vec![vec![1, 0, 1], vec![1, 2, 1], vec![0, 1, 0]];

    let rotted = item.rot(grid.as_mut());
    assert_eq!(rotted.len(), 3);
  }
}

#[cfg(test)]
mod oranges_rotting {
  use super::*;
  #[test]
  fn test_case() {
    let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
    let result = Solution::oranges_rotting(grid);
    assert_eq!(result, 4);

    let grid = vec![vec![0, 2]];
    let result = Solution::oranges_rotting(grid);
    assert_eq!(result, 0);

    let grid = vec![vec![0]];
    let result = Solution::oranges_rotting(grid);
    assert_eq!(result, 0);
  }

  #[test]
  fn a_grid_cannot_be_rotted_completely_sr_negative_1() {
    let grid = vec![vec![1, 0, 1], vec![0, 2, 1], vec![1, 1, 1]];
    let result = Solution::oranges_rotting(grid);
    assert_eq!(result, -1);
  }
}
