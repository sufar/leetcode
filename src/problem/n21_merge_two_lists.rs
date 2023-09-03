// https://leetcode.cn/problems/merge-two-sorted-lists/description/
//
// 21. 合并两个有序链表
//
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (list1, list2);
        let mut result: Box<ListNode> = Box::new(ListNode::new(0));

        while l1.is_some() || l2.is_some() {
            if let Some(x) = l1 {
                if let Some(y) = l2 {
                    if x.val < y.val {
                        result.next = Some(Box::new(ListNode::new(x.val)));
                        l1 = x.next;
                    } else {
                        result.next = Some(Box::new(ListNode::new(y.val)));
                        l2 = y.next;
                    }
                } else {
                    result.next = Some(Box::new(ListNode::new(x.val)));
                    l1 = x.next;
                }
            } else {
                if let Some(x) = l2 {
                    result.next = Some(Box::new(ListNode::new(x.val)));
                    l2 = x.next;
                }
            }
        }

        return result.next;
    }
}
