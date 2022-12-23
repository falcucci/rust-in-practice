//  You are given two non-empty linked lists representing two non-negative
//  integers. The digits are stored in reverse order,
//  and each of their nodes contains a single digit.
//  Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero,
// except the number 0 itself.
//
// Example 1:
// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
//
// Example 2:
// Input: l1 = [0], l2 = [0]
// Output: [0]

fn from_vec_to_string(vector: Vec<i32>) -> String {
  vector.iter().map(ToString::to_string).collect()
}

fn from_string_to_int(stringified: String) -> u32 {
  stringified.parse::<u32>().unwrap()
}

fn sum_reverse_linked_list(first_list: Vec<i32>, second_list: Vec<i32>) -> Vec<u32> {
  let mut first_list_clone: Vec<i32> = first_list.clone();
  let mut second_list_clone: Vec<i32> = second_list.clone();

  first_list_clone.reverse();
  second_list_clone.reverse();

  let first_list_stringified: String = from_vec_to_string(first_list_clone);
  let second_list_stringified: String = from_vec_to_string(second_list_clone);
  let first_integer: u32 = from_string_to_int(first_list_stringified);
  let second_integer: u32 = from_string_to_int(second_list_stringified);
  let sum: u32 = first_integer + second_integer;

  let mut result: Vec<u32> = sum
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

  result.reverse();
  result
}

fn main() {
  let l1: Vec<i32> = vec![2, 4, 3];
  let l2: Vec<i32> = vec![5, 6, 4];
  let result: Vec<u32> = sum_reverse_linked_list(l1, l2);
  println!("{:?}", result);
}

#[cfg(test)]
mod tests {
  use super::sum_reverse_linked_list;

  #[test]
  fn sum_two_case_one() {
    // Input: l1 = [2,4,3], l2 = [5,6,4]
    // Output: [7,0,8]
    // Explanation: 342 + 465 = 807.
    let l1: Vec<i32> = vec![2, 4, 3];
    let l2: Vec<i32> = vec![5, 6, 4];
    let result: Vec<u32> = vec![7, 0, 8];
    assert_eq!(result, sum_reverse_linked_list(l1, l2));
  }

  #[test]
  fn sum_two_case_two() {
    // Example 2:
    // Input: l1 = [0], l2 = [0]
    // Output: [0]
    let l1: Vec<i32> = vec![0];
    let l2: Vec<i32> = vec![0];
    let result: Vec<u32> = vec![0];
    assert_eq!(result, sum_reverse_linked_list(l1, l2));
  }

  #[test]
  fn sum_two_case_three() {
    // Example 3:
    // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    // Output: [8,9,9,9,0,0,0,1]
    let l1: Vec<i32> = vec![9, 9, 9, 9, 9, 9, 9];
    let l2: Vec<i32> = vec![9, 9, 9, 9];
    let result: Vec<u32> = vec![8, 9, 9, 9, 0, 0, 0, 1];
    assert_eq!(result, sum_reverse_linked_list(l1, l2));
  }

  #[test]
  fn sum_two_case_four() {
    // Example 4:
    // Input: l1 = [2,4,9], l2 = [5,6,4,9]
    // Output: [7,0,4,0,1]
    let l1: Vec<i32> = vec![2, 4, 9];
    let l2: Vec<i32> = vec![5, 6, 4, 9];
    let result: Vec<u32> = vec![7, 0, 4, 0, 1];
    assert_eq!(result, sum_reverse_linked_list(l1, l2));
  }

  #[test]
  fn sum_two_case_five() {
    // Example 5:
    // Input: l1 = [1], l2 = [9,9]
    // Output: [0,0,1]
    let l1: Vec<i32> = vec![1];
    let l2: Vec<i32> = vec![9, 9];
    let result: Vec<u32> = vec![0, 0, 1];
    assert_eq!(result, sum_reverse_linked_list(l1, l2));
  }
} /* Tests */
