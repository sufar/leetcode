package com.vorise.leetcode;

import java.util.Arrays;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

import org.junit.Assert;
import org.junit.Test;

/**
 * https://leetcode.cn/problems/reverse-vowels-of-a-string/description/
 *
 * 345. 反转字符串中的元音字母
 * 给你一个字符串 s ，仅反转字符串中的所有元音字母，并返回结果字符串。
 * 
 * 元音字母包括 'a'、'e'、'i'、'o'、'u'，且可能以大小写两种形式出现不止一次。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 输入：s = "hello"
 * 输出："holle"
 * 示例 2：
 * 
 * 输入：s = "leetcode"
 * 输出："leotcede"
 * 
 * 
 * 提示：
 * 
 * 1 <= s.length <= 3 * 105
 * s 由 可打印的 ASCII 字符组成
 *
 * @author zugle on 2023-08-23
 */
public class n345_reverseVowels {

    class Solution {
        public String reverseVowels(String s) {
            // 使用双指针，一个指针从头向尾遍历，一个指针从尾到头遍历，当两个指针都遍历到元音字符时，交换这两个元音字符。
            // 为了快速判断一个字符是不是元音字符，我们将全部元音字符添加到集合 HashSet 中，从而以 O(1) 的时间复杂度进行该操作。
            if (s == null) {
                return null;
            }
            Set<Character> chs = new HashSet<>(Arrays.asList('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'));
            char[] result = new char[s.length()];
            int left = 0;
            int right = s.length() - 1;
            while (left <= right) {
                char l = s.charAt(left);
                char r = s.charAt(right);
                if (!chs.contains(l)) {
                    result[left++] = l;
                } else if (!chs.contains(r)) {
                    result[right--] = r;
                } else {
                    result[left++] = r;
                    result[right--] = l;
                }
            }
            return new String(result);
        }
    }

    @Test
    public void test() {
        String s = "hello";
        Assert.assertEquals("holle", new Solution().reverseVowels(s));
    }
}
