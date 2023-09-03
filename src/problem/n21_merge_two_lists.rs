// https://leetcode.cn/problems/merge-two-sorted-lists/description/
//
// 21. 合并两个有序链表
//
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (list1, list2);
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));

        while l1.is_some() || l2.is_some() {
            let mut num1 = 0;
            if let Some(ref x) = l1 {
                num1 = x.val;
            }
            let mut num2 = 0;
            if let Some(ref x) = l2 {
                num2 = x.val;
            }

            if num1 < num2 {
                if let Some(x) = l1 {
                    l1 = x.next;
                }
                result.next = Some(Box::new(ListNode::new(num1)));
            } else {
                if let Some(x) = l2 {
                    l2 = x.next;
                }
                result.next = Some(Box::new(ListNode::new(num2)));
            }
        }

        return result.next;

    }
}
