package com.vorise.leetcode;

/**
 * https://leetcode.cn/problems/longest-palindrome/description/
 *
 * 409. 最长回文串
 *
 * 给定一个包含大写字母和小写字母的字符串 s ，返回 通过这些字母构造成的 最长的回文串 。
 * 
 * 在构造过程中，请注意 区分大小写 。比如 "Aa" 不能当做一个回文字符串。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 输入:s = "abccccdd"
 * 输出:7
 * 解释:
 * 我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
 * 示例 2:
 * 
 * 输入:s = "a"
 * 输出:1
 * 示例 3：
 * 
 * 输入:s = "aaaaaccc"
 * 输出:7
 * 
 * 
 * 提示:
 * 
 * 1 <= s.length <= 2000
 * s 只由小写 和/或 大写英文字母组成
 *
 * @author zugle on 2023-09-03
 */
public class n409_longestPalindrome {

    /**
     *
     * 使用长度为256的整型数组来统计每个字符出现的个数，每个字符有偶数个可以用来构成回文字符串。
     * 
     * 因为回文字符串最中间的那个字符可以单独出现，所以如果有单独的字符就把它放到最中间。
     */
    class Solution {
        public int longestPalindrome(String s) {
            int[] chs = new int[256];
            for (char c : s.toCharArray()) {
                chs[c]++;
            }

            int num = 0;
            for (int ch : chs) {
                num += (ch / 2) * 2;
            }

            if (num < s.length()) {
                num++;
            }
            return num;
        }
    }
}
