// 9. 回文数
//
// 给你一个整数 x ，如果 x 是一个回文整数，返回 true ；否则，返回 false 。
//
// 回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
//
// 例如，121 是回文，而 123 不是。
//
//
// 示例 1：
//
// 输入：x = 121
// 输出：true
// 示例 2：
//
// 输入：x = -121
// 输出：false
// 解释：从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
// 示例 3：
//
// 输入：x = 10
// 输出：false
// 解释：从右向左读, 为 01 。因此它不是一个回文数。
//
//
// 提示：
//
// -231 <= x <= 231 - 1
//
//
// 进阶：你能不将整数转为字符串来解决这个问题吗？
#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }
        if x % 10 == 0 {
            return false;
        }
        let mut left = x;
        let mut right = 0;
        while right < left {
            // 1234321 % 10 = 1 
            // left: 1234321 / 10 = 123432
            // right: 0*10 + 1 = 1
            //
            // 123432 % 10 = 2
            // left: 123432 / 10 = 12343
            // right: 1 * 10 + 2 = 12
            let a = left % 10; // 个位
            left = left / 10;
            right = right * 10 + a;
        }
        return left == right || left == right / 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let r = Solution::is_palindrome(1245421);
        assert_eq!(r, true);
    }

    #[test]
    fn test_divide() {
        let r = 5 / 2;
        assert_eq!(r, 2);
    }
    
    #[test]
    fn test_mod() {
        let r = 5 % 2;
        assert_eq!(r, 1);
    }

    #[test]
    fn test_chars() {
        let s = "12345";
        let mut c = s.chars();
        assert_eq!('1', c.next().unwrap());
        assert_eq!('2', c.next().unwrap());
        assert_eq!("345", c.as_str());
    }
}
