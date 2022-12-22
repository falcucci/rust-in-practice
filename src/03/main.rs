// Given two sorted arrays nums1 and nums2 of size m and n respectively,
// return the median of the two sorted arrays.
//
// The overall run time complexity should be O(log (m+n)).
//
// Example 1:
//
// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
// Example 2:
//
// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
//
//
// Constraints:
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106

struct Solution {}

fn merge_vec(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
  let mut full: Vec<i32> = vec1;
  full.extend(vec2);
  full.sort();
}

impl Solution {
  pub fn find_median_sorted_arrays(_l1: Vec<i32>, _l2: Vec<i32>) -> f64 {
    let full: Vec<i32> = merge_vec(_l1, _l2);
    let len: usize = full.len();
    let mid: usize = len / 2;
    let mid_s: i32 = full[mid - 1] + full[mid];
    let rest: i32 = mid_s % 2;
    match rest {
      0 => mid_s as f64 / 2.0,
      _ => full[mid] as f64,
    }
  }
}

fn main() {
  let l1: Vec<i32> = vec![1, 3];
  let l2: Vec<i32> = vec![2];
  let result: f64 = Solution::find_median_sorted_arrays(l1, l2);
  println!("result: {}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn first_scenario_passing_odd_merged_vectors() {
    let l1: Vec<i32> = vec![1, 3];
    let l2: Vec<i32> = vec![2];
    let result: f64 = Solution::find_median_sorted_arrays(l1, l2);
    assert_eq!(result, 2.0);
  }
}
