package com.vorise.leetcode;

import org.junit.Assert;
import org.junit.Test;

/**
 * https://leetcode.cn/problems/valid-palindrome-ii/description/
 *
 * 680. 验证回文串 II
 *
 * 给你一个字符串 s，最多 可以从中删除一个字符。
 * 
 * 请你判断 s 是否能成为回文字符串：如果能，返回 true ；否则，返回 false 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：s = "aba"
 * 输出：true
 * 示例 2：
 * 
 * 输入：s = "abca"
 * 输出：true
 * 解释：你可以删除字符 'c' 。
 * 示例 3：
 * 
 * 输入：s = "abc"
 * 输出：false
 * 
 * 
 * 提示：
 * 
 * 1 <= s.length <= 105
 * s 由小写英文字母组成
 *
 * @author zugle on 2023-08-25
 */
public class n680_validPalindrome {

    @Test
    public void test() {
        String s = "deeee";
        boolean result = new Solution().validPalindrome(s);
        Assert.assertEquals(result, true);
    }
}

class Solution {
    /**
     * 本题的关键是处理删除一个字符。在使用双指针遍历字符串时，如果出现两个指针指向的字符不相等的情况，我们就试着删除一个字符，再判断删除完之后的字符串是否是回文字符串。
     * 
     * 在判断是否为回文字符串时，我们不需要判断整个字符串，因为左指针左边和右指针右边的字符之前已经判断过具有对称性质，所以只需要判断中间的子字符串即可。
     * 
     * 在试着删除字符时，我们既可以删除左指针指向的字符，也可以删除右指针指向的字符。
     */
    public boolean validPalindrome(String s) {
        for (int i = 0, j = s.length() - 1; i < j; i++, j--) {
            if (s.charAt(i) != s.charAt(j)) {
                return (isPalindrome(s, i + 1, j) || isPalindrome(s, i, j - 1));
            }
        }
        return true;
    }

    private boolean isPalindrome(String s, int start, int end) {
        while (start < end) {
            if (s.charAt(start) != s.charAt(end)) {
                return false;
            }
            start++;
            end--;
        }
        return true;
    }
}
