// https://leetcode.cn/problems/add-two-numbers/
// https://leetcode.cn/problems/add-two-numbers/solutions/428924/rust-by-ruislan-149/

// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
//
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
//
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
//
//  
//
// 示例 1：
//
//
// 输入：l1 = [2,4,3], l2 = [5,6,4]
// 输出：[7,0,8]
// 解释：342 + 465 = 807.
// 示例 2：
//
// 输入：l1 = [0], l2 = [0]
// 输出：[0]
// 示例 3：
//
// 输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// 输出：[8,9,9,9,0,0,0,1]
//  
//
// 提示：
//
// 每个链表中的节点数在范围 [1, 100] 内
// 0 <= Node.val <= 9
// 题目数据保证列表表示的数字不含前导零

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

    fn set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }

    fn push(&mut self, ele: i32) {
        let new_node = ListNode::new(ele);
        self.get_last().set_next(new_node);
    }

    fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut carry = 0; // 进位
        let (mut l1, mut l2, mut cur) = (l1, l2, head.as_mut());
        
        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(ref node) = l2 {
                sum += node.val;
            }

            carry = sum / 10;
            sum = sum % 10;

            cur.next = Some(Box::new(ListNode::new(sum)));
            cur = cur.next.as_mut().unwrap();
        }

        if carry > 0 {
            cur.next = Some(Box::new(ListNode::new(carry)));
        }
        
        head.next



        // let mut num1 = 0;
        // let mut num2 = 0;
        // let mut index1 = 0;
        // let mut index2 = 0;
        // let mut node = l1;
        // while let Some(n) = node {
        //     num1 = num1 + n.val * 10i32.pow(index1);
        //     node = n.next;
        //     index1 += 1;
        // }
        //
        // node = l2;
        // while let Some(n) = node {
        //     num2 = num2 + n.val * 10i32.pow(index2);
        //     node = n.next;
        //     index2 += 1;
        // }
        // let mut total = num1 + num2;
        // let mut result = Some(Box::new(ListNode::new(0)));
        // loop {
        //     let temp = total % 10;
        //     total = total / 10;
        //     result.as_mut().unwrap().next = Some(Box::new(ListNode::new(temp)));
        //     let next = result.unwrap().next;
        //     result = next;
        //     if total == 0 {
        //         return result;
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    #[test]
    fn test_add_two_numbers() {
        // let l1: Option<Box<ListNode>> = Some(Box::new());
        let mut l1 = ListNode::new(2);
        l1.push(4);
        l1.push(3);
        // println!("{}", l1.get_last().val);

        let mut l2 = ListNode::new(5);
        l2.push(6);
        l2.push(4);

        let result = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
        println!("{}", result.unwrap().next.unwrap().val);
    }
}
