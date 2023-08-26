package com.vorise.leetcode;

import java.util.LinkedList;
import java.util.List;

import org.junit.Assert;
import org.junit.Test;

// Definition for singly-linked list.

public class Loop {

    @Test
    public void test() {

        Node node = new Node(1).setNext(
                new Node(2).setNext(new Node(3).setNext(new Node(4).setNext(new Node(5).setNext(new Node(2))))));

        boolean result = new Solution().isLoop(node);
        Assert.assertEquals(result, true);
    }
}

class Node {
    private int data;
    private Node next;

    public Node(int d) {
        this.data = d;
        this.next = null;
    }

    public Node getNext() {
        return next;
    }

    public Node setNext(Node next) {
        this.next = next;
        return this;
    }
}

class Solution {

    public boolean isLoop(Node node) {
        Node slow = node;
        if (slow == null) {
            return false;
        }
        Node fast = node.getNext();
        if (fast == null) {
            return false;
        }
        while (slow != null && fast != null && fast.getNext() != null) {
            if (slow == fast) {
                return true;
            }
            slow = slow.getNext();
            fast = fast.getNext().getNext();
        }
        return false;
    }
}

// How to know this linked list is cycled or not?
// pseudo code:
// var llist = new LinkedList();
// llist.push(1);
// llist.push(2);
// llist.push(3);
// llist.push(4);
// llist.push(5);
// llist.push(2);
