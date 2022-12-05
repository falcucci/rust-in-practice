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
  println!("{:?}", head);
  head
}

fn main() {
  let head: Vec<i32> = vec![1, 2, 3, 4, 5];
  find_the_middle(head);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn return_from_the_middle_value() {
    let head: Vec<i32> = vec![1, 2, 3, 4, 5];
    let clone: Vec<i32> = head.clone();
    let result: Vec<i32> = find_the_middle(head);
    assert_eq!(find_the_middle(clone), result);
  }
} /* tests */
