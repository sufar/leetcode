package com.vorise.leetcode;

import org.junit.Test;

public class n2_addTwoNumbers {
    // Definition for singly-linked list.
    public static class ListNode {
        int val;
        ListNode next;

        ListNode() {
        }

        ListNode(int val) {
            this.val = val;
        }

        ListNode(int val, ListNode next) {
            this.val = val;
            this.next = next;
        }
    }

    public static class Solution {
        public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
            ListNode pre = new ListNode(0); // 指向头节点
            ListNode cur = pre;// 当前节点
            int carry = 0; // 进位

            while (l1 != null || l2 != null) {
                int x = l1 == null ? 0 : l1.val;
                int y = l2 == null ? 0 : l2.val;
                int sum = x + y + carry;
                carry = sum / 10; // 进位变化
                sum = sum % 10;  // 实际存入链表的值

                cur.next = new ListNode(sum);

                cur = cur.next;

                if (l1 != null) {
                    l1 = l1.next;
                }

                if (l2 != null) {
                    l2 = l2.next;
                }
            }

            // 进位为1，添加节点
            if (carry == 1) {
                cur.next = new ListNode(carry);
            }

            return pre.next;
        }
    }

    @Test
    public void test() {
        Solution solution = new Solution();
        ListNode l1 = new ListNode(2);
        ListNode l2 = new ListNode(4);
        ListNode l3 = new ListNode(3);
        l1.next = l2;
        l2.next = l3;

        ListNode r1 = new ListNode(5);
        ListNode r2 = new ListNode(6);
        ListNode r3 = new ListNode(4);
        r1.next = r2;
        r2.next = r3;

        ListNode result = solution.addTwoNumbers(l1, r1);
        System.out.println(result.val);
    }
}
