package com.vorise.leetcode;

import org.junit.Assert;
import org.junit.Test;

/**
 * <a href=
 * "https://leetcode.cn/problems/longest-palindromic-substring/description/">5.
 * 最长回文子串</a>
 *
 * @author zugle on 2023-08-14
 */
public class n5_longestPalindrome {

    @Test
    public void testLongestPalindrome() {
        String s = "babad";
        String result = longestPalindrome(s);
        Assert.assertEquals(result, "bab");
    }

    public String longestPalindrome(String s) {
        int length = s.length();
        if (length < 2) {
            return s;
        }
        int maxLength = 1; // 最大回文字符串
        int beigin = 0; // 最大回文数起始位置

        char[] data = s.toCharArray();

        for (int i = 0; i < length - 1; i++) {
            for (int j = i + 1; j < length; j++) {
                if (j - i + 1 > maxLength && isPalindrome(data, i, j)) {
                    maxLength = j - i + 1;
                    beigin = i;
                }
            }
        }

        return s.substring(beigin, beigin + maxLength);
    }

    /**
     * 验证s[left..right]是否为回文串
     */
    public boolean isPalindrome(char[] data, int left, int right) {
        while (left < right) {
            if (data[left] != data[right]) {
                return false;
            }
            left++;
            right--;
        }
        return true;
    }

    /////////////////////////////// 中心扩散 /////////////////////////

    public int expandAroundCenter(char[] data, int left, int right) {
        int len = data.length;

        int i = left;
        int j = right;
        while (i >= 0 && j < len) {
            if (data[i] == data[j]) {
                i--;
                j++;
            } else {
                break;
            }
        }
        return 
    }
}
