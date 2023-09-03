// https://leetcode.cn/problems/reverse-integer/
//
// 7. 整数反转
//
// 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
//
// 如果反转后整数超过 32 位的有符号整数的范围 [−231,  231 − 1] ，就返回 0。
//
// 假设环境不允许存储 64 位整数（有符号或无符号）。
//
//
// 示例 1：
//
// 输入：x = 123
// 输出：321
// 示例 2：
//
// 输入：x = -123
// 输出：-321
// 示例 3：
//
// 输入：x = 120
// 输出：21
// 示例 4：
//
// 输入：x = 0
// 输出：0
//
//
// 提示：
//
// -231 <= x <= 231 - 1

struct Solution {}

impl Solution {
    // %10得到个位数，/10相当于向由移位
    pub fn reverse(x: i32) -> i32 {
        let mut r1 = x;
        let mut r2 = 0;
        while r1 != 0 {
            if r2 > i32::max_value() / 10 || r2  < i32::min_value() / 10 {
                return 0;
            }
            r2 = r2 * 10 + r1 % 10;
            r1 = r1 / 10;
        }
        return r2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("123 / 10: {}", 123 / 10);
        println!("123 % 10: {}", 123 % 10);
        println!("1 / 10: {}", 1 / 10);
        println!("1 % 10: {}", 1 % 10);
        println!("20 / 10: {}", 20 / 10);
        assert_eq!(Solution::reverse(1234), 4321);
        assert_eq!(Solution::reverse(-1234), -4321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
