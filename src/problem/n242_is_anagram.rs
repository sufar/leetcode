// https://leetcode.cn/problems/valid-anagram/description/
//
// 242. 有效的字母异位词
//
// 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
//
// 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
//
//  
//
// 示例 1:
//
// 输入: s = "anagram", t = "nagaram"
// 输出: true
// 示例 2:
//
// 输入: s = "rat", t = "car"
// 输出: false
//  
//
// 提示:
//
// 1 <= s.length, t.length <= 5 * 104
// s 和 t 仅包含小写字母
//  
//
// 进阶: 如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？

struct Solution {}

impl Solution {

    // 由于本题的字符串只包含 26 个小写字符，因此可以使用长度为 26 的整型数组对字符串出现的字符进行统计，不再使用 HashMap。
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut buffer = [0i32; 26];
        let mut chars = s.chars();
        while let Some(ch) = chars.next() {
            if ch < 'a' || ch > 'z' {
                continue;
            }
            let index = ch as u32 - 'a' as u32;
            let index = index as usize;
            buffer[index] = buffer[index] + 1;
        }
        let mut chars = t.chars();
        while let Some(ch) = chars.next() {
            if ch < 'a' || ch > 'z' {
                continue;
            }
            let index = ch as u32 - 'a' as u32;
            let index = index as usize;
            buffer[index] = buffer[index] - 1;
        }
        for item in buffer {
            if item != 0 {
                return false;
            }
        }

        return true;
        
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        println!("{}", 'a' as u32);
        let s = "anagram".to_string(); 
        let t = "nagaram".to_string();


        assert_eq!(Solution::is_anagram(s, t), true);
    }
}
