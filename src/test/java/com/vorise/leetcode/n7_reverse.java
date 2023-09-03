package com.vorise.leetcode;

/**
 * https://leetcode.cn/problems/reverse-integer/
 *
 * 
 * 7. 整数反转
 * 
 * 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
 * 
 * 如果反转后整数超过 32 位的有符号整数的范围 [−231, 231 − 1] ，就返回 0。
 * 
 * 假设环境不允许存储 64 位整数（有符号或无符号）。
 * 
 * 
 * 示例 1：
 * 
 * 输入：x = 123
 * 输出：321
 * 示例 2：
 * 
 * 输入：x = -123
 * 输出：-321
 * 示例 3：
 * 
 * 输入：x = 120
 * 输出：21
 * 示例 4：
 * 
 * 输入：x = 0
 * 输出：0
 * 
 * 
 * 提示：
 * 
 * -231 <= x <= 231 - 1
 *
 * @author zugle on 2023-09-03
 */
public class n7_reverse {

    class Solution {

        /**
         * 通过 %10和/10进行移位
         */
        public int reverse(int x) {
            int result = 0;
            while (x != 0) {
                if (result < Integer.MIN_VALUE / 10 || result > Integer.MAX_VALUE / 10) {
                    return 0;
                }
                result = result * 10 + x % 10;
                x = x / 10;
            }

            return result;
        }
   }
}
