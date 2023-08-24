package com.vorise.leetcode;

import org.junit.Assert;
import org.junit.Test;

/**
 * https://leetcode.cn/problems/sum-of-square-numbers/description/
 *
 * 633. 平方数之和
 * 
 * 给定一个非负整数 c ，你要判断是否存在两个整数 a 和 b，使得 a2 + b2 = c 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：c = 5
 * 输出：true
 * 解释：1 * 1 + 2 * 2 = 5
 * 示例 2：
 * 
 * 输入：c = 3
 * 输出：false
 * 
 * 
 * 提示：
 * 
 * 0 <= c <= 231 - 1
 * 
 * @author zugle on 2023-08-23
 */
public class n633_judge_square_sum {

    class Solution {
        public boolean judgeSquareSum(int c) {
            if (c < 0) {
                return false;
            }
            double num = Math.sqrt(c);
            long left = 0;
            long right = (int) num;
            while (left <= right) {
                long result = left * left + right * right;
                if (result > c) {
                    right--;
                } else if (result == c) {
                    return true;
                } else {
                    left++;
                }
            }

            return false;
        }
    }

    @Test
    public void test1() {
        int c = 0;
        Assert.assertEquals(true, new Solution().judgeSquareSum(c));
    }

    @Test
    public void test2() {
        int c = 3;
        Assert.assertEquals(false, new Solution().judgeSquareSum(c));
    }

    @Test
    public void test3() {
        int c = 5;
        Assert.assertEquals(true, new Solution().judgeSquareSum(c));
    }

    @Test
    public void test4() {
        int c = 2147483646;
        Assert.assertEquals(false, new Solution().judgeSquareSum(c));
    }
}
