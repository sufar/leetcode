// https://leetcode.cn/problems/reverse-vowels-of-a-string/submissions/

// 345. 反转字符串中的元音字母
//
// 给你一个字符串 s ，仅反转字符串中的所有元音字母，并返回结果字符串。
//
// 元音字母包括 'a'、'e'、'i'、'o'、'u'，且可能以大小写两种形式出现不止一次。
//
//
//
// 示例 1：
//
// 输入：s = "hello"
// 输出："holle"
// 示例 2：
//
// 输入：s = "leetcode"
// 输出："leotcede"
//
//
// 提示：
//
// 1 <= s.length <= 3 * 105
// s 由 可打印的 ASCII 字符组成

struct Solution {}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        // 首尾双指针
        if s.len() == 0 {
            return "".to_string();
        }
        let vowels = Box::new(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']);
        let mut result: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if !vowels.contains(&result[left]) {
                left += 1;
            } else if !vowels.contains(&result[right]) {
                right -= 1;
            } else {
                result.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test() {
        let s = "n.".to_string();

        let result = Solution::reverse_vowels(s);
        assert_eq!("n.".to_string(), result);
    }
}
