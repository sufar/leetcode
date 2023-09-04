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

    // rust迭代实现
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head: Box<ListNode> = Box::new(ListNode::new(0));
        let (mut l1, mut l2, mut cur) = (list1, list2, head.as_mut());

        while l1.is_some() && l2.is_some() {
            let (p1, p2) = (l1.as_mut().unwrap(), l2.as_mut().unwrap());
            if p1.val < p2.val {
                cur.next = Some(Box::new(ListNode::new(p1.val)));
                l1 = p1.next.take();
            } else {
                cur.next = Some(Box::new(ListNode::new(p2.val)));
                l2 = p2.next.take();
            }
            cur = cur.next.as_mut().unwrap();
        }

        cur.next = l1.or(l2);

        return head.next;
    }
}
