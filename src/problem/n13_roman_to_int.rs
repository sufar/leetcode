// 罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
//
// 字符          数值
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// 例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
//
// 通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
//
// I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
// X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。
// C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
// 给定一个罗马数字，将其转换成整数。
//
//
//
// 示例 1:
//
// 输入: s = "III"
// 输出: 3
// 示例 2:
//
// 输入: s = "IV"
// 输出: 4
// 示例 3:
//
// 输入: s = "IX"
// 输出: 9
// 示例 4:
//
// 输入: s = "LVIII"
// 输出: 58
// 解释: L = 50, V= 5, III = 3.
// 示例 5:
//
// 输入: s = "MCMXCIV"
// 输出: 1994
// 解释: M = 1000, CM = 900, XC = 90, IV = 4.
//
//
// 提示：
//
// 1 <= s.length <= 15
// s 仅含字符 ('I', 'V', 'X', 'L', 'C', 'D', 'M')
// 题目数据保证 s 是一个有效的罗马数字，且表示整数在范围 [1, 3999] 内
// 题目所给测试用例皆符合罗马数字书写规则，不会出现跨位等情况。
// IL 和 IM 这样的例子并不符合题目要求，49 应该写作 XLIX，999 应该写作 CMXCIX 。
// 关于罗马数字的详尽书写规则，可以参考 罗马数字 - Mathematics 。
#[allow(dead_code)]
struct Solution {}

// 每一个罗马数字都去做加法，如果遇到特殊的如IV，如果正常是1+5=6，实际是1+3=4
// 所以判断前一个字符，修正当前字符为+3，而不是+5
#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut cs = s.chars();
        let mut p = cs.next().unwrap();
        let mut r = 0;
        r += Solution::parse(p, None);

        for ele in cs {
            r += Solution::parse(ele, Some(p));
            p = ele;
        }
        return r;
    }

    fn parse(ele: char, p: Option<char>) -> i32 {
        match ele {
            'I' => 1,
            'V' => {
                if let Some(p) = p  {
                    if p == 'I' {
                        3
                    } else {
                        5
                    }
                } else {
                    5
                }
            },
            'X' => {
                if let Some(p) = p {
                    if p == 'I' {
                        8
                    } else {
                        10
                    }
                } else {
                    10
                }
            },
            'L' => {
                if let Some(p) = p {
                    if p == 'X' {
                        30
                    } else {
                        50
                    }
                } else {
                    50
                }
            },
            'C' => {
                if let Some(p) = p {
                    if p == 'X' {
                        80
                    } else {
                        100
                    }
                } else {
                    100
                }
            },
            'D' => {
                if let Some(p) = p {
                    if p == 'C' {
                        300
                    } else {
                        500
                    }
                } else {
                    500
                }
            },
            'M' => {
                if let Some(p) = p {
                    if p == 'C' {
                        800
                    } else {
                        1000
                    }
                } else {
                    1000
                }
            },
            _ => 0, 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3() {
        let s = "III".to_string();
        assert_eq!(Solution::roman_to_int(s), 3);
    }

    #[test]
    fn test4() {
        let s = "IV".to_string();
        assert_eq!(Solution::roman_to_int(s), 4);
    }

    #[test]
    fn test9() {
        let s = "IX".to_string();
        assert_eq!(Solution::roman_to_int(s), 9);
    }

    #[test]
    fn test58() {
        let s = "LVIII".to_string();
        assert_eq!(Solution::roman_to_int(s), 58);
    }

    #[test]
    fn test1994() {
        let s = "MCMXCIV".to_string();
        assert_eq!(Solution::roman_to_int(s), 1994);
    }
}
