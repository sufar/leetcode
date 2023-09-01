package com.vorise.leetcode;

import org.junit.Assert;
import org.junit.Test;

/**
 *
 * https://leetcode.cn/problems/valid-anagram/description/
 *
 * 242. 有效的字母异位词
 * 
 * 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
 * 
 * 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 输入: s = "anagram", t = "nagaram"
 * 输出: true
 * 示例 2:
 * 
 * 输入: s = "rat", t = "car"
 * 输出: false
 * 
 * 
 * 提示:
 * 
 * 1 <= s.length, t.length <= 5 * 104
 * s 和 t 仅包含小写字母
 * 
 * 
 * 进阶: 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？
 * 
 * @author zugle on 2023-09-01
 */
public class n242_isAnagram {

    @Test
    public void test() {
        String s = "anagram", t = "nagaram";
        boolean result = new Solution().isAnagram(s, t);
        Assert.assertEquals(result, true);
    }

    class Solution {
        // 由于本题的字符串只包含 26 个小写字符，因此可以使用长度为 26 的整型数组对字符串出现的字符进行统计，不再使用 HashMap。
        public boolean isAnagram(String s, String t) {
            int[] buffer = new int[26];
            s.chars().forEach(c -> {
                if (c < 'a' || c > 'z') {
                    return;
                }
                buffer[c - 'a']++;
            });
            t.chars().forEach(c -> {
                if (c < 'a' || c > 'z') {
                    return;
                }
                buffer[c - 'a']--;
            });
            for (int item : buffer) {
                if (item != 0) {
                    return false;
                }
            }
            return true;
        }
    }
}
