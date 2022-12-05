// Given the head of a singly linked list,
// return the middle node of the linked list.
//
// If there are two middle nodes, return the second middle node.
// Input: head = [1,2,3,4,5]
// Output: [3,4,5]
// Explanation: The middle node of the list is node 3.
// Example 2:
//
//
// Input: head = [1,2,3,4,5,6]
// Output: [4,5,6]
// Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.

pub fn find_the_middle(head: Vec<i32>) -> Vec<i32> {
  let len: usize = head.len();
  let initial: usize = len / 2;
  let result: &[i32] = &head[initial..len];
  result.to_vec()
}

fn main() {
  let head: Vec<i32> = vec![1, 2, 3, 4, 5];
  find_the_middle(head);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn return_from_the_middle_even_vector() {
    let head: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i32> = find_the_middle(head);
    let clone: Vec<i32> = vec![4, 5, 6];
    assert_eq!(result, clone);
  }

  #[test]
  fn return_from_the_middle_odd_elements() {
    let head: Vec<i32> = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = find_the_middle(head);
    let clone: Vec<i32> = vec![3, 4, 5];
    assert_eq!(result, clone);
  }
} /* tests */
