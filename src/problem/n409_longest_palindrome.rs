// https://leetcode.cn/problems/longest-palindrome/description/
//
// 409. 最长回文串
//
// 给定一个包含大写字母和小写字母的字符串 s ，返回 通过这些字母构造成的 最长的回文串 。
//
// 在构造过程中，请注意 区分大小写 。比如 "Aa" 不能当做一个回文字符串。
//
//
//
// 示例 1:
//
// 输入:s = "abccccdd"
// 输出:7
// 解释:
// 我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
// 示例 2:
//
// 输入:s = "a"
// 输出:1
// 示例 3：
//
// 输入:s = "aaaaaccc"
// 输出:7
//
//
// 提示:
//
// 1 <= s.length <= 2000
// s 只由小写 和/或 大写英文字母组成

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut buffer = [0i32; 52];
        let mut chs = s.chars();
        while let Some(ch) = chs.next() {
            if ch >= 'a' && ch <= 'z' {
                let index = ch as u32 - 6 - 'A' as u32;
                buffer[index as usize] = buffer[index as usize] + 1;
            } else if ch >= 'A' && ch <= 'Z' {
                let index = ch as u32 - 'A' as u32;
                buffer[index as usize] = buffer[index as usize] + 1;
            } else {
                continue;
            }
        }
        let mut result = 0;
        let mut middle = false;
        for item in buffer {
            if item / 2 > 0 {
                if item % 2 == 0 {
                    result = result + item;
                } else {
                    result = result + item - 1;
                }
            }

            if item % 2 == 1 {
                middle = true;
            }
        }
        if middle {
            result = result + 1;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        println!(
            "a: {}, z: {}, A: {}, Z: {}",
            'a' as u32, 'z' as u32, 'A' as u32, 'Z' as u32
        );

        let s = "aaaaaccc".to_string();
        assert_eq!(Solution::longest_palindrome(s), 7);
    }
}
