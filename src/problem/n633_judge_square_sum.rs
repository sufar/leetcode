// https://leetcode.cn/problems/sum-of-square-numbers/description/
//
// 633. 平方数之和
//
// 给定一个非负整数 c ，你要判断是否存在两个整数 a 和 b，使得 a2 + b2 = c 。
//
// 示例 1：
//
// 输入：c = 5
// 输出：true
// 解释：1 * 1 + 2 * 2 = 5
// 示例 2：
//
// 输入：c = 3
// 输出：false
//  
//
// 提示：
//
// 0 <= c <= 231 - 1


struct Solution {}

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {

        // 首尾双指针
        if c < 0 {
            return false;
        }
        let mut left = 0;
        let mut right = (c as f64).sqrt() as i64;

        while left <= right {
            let result = left * left + right * right;

            if result == c as i64 {
                return true;
            } else if result < c as i64 {
                left += 1;
            } else {
                right -= 1;
            }
        }

        return false;
    }
}
