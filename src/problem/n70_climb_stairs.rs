// 70. 爬楼梯
// 
// 假设你正在爬楼梯。需要 n 阶你才能到达楼顶。
//
// 每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？
//
//  
//
// 示例 1：
//
// 输入：n = 2
// 输出：2
// 解释：有两种方法可以爬到楼顶。
// 1. 1 阶 + 1 阶
// 2. 2 阶
// 示例 2：
//
// 输入：n = 3
// 输出：3
// 解释：有三种方法可以爬到楼顶。
// 1. 1 阶 + 1 阶 + 1 阶
// 2. 1 阶 + 2 阶
// 3. 2 阶 + 1 阶
//  
//
// 提示：
//
// 1 <= n <= 45
#[allow(dead_code)]
struct Solution {}

// 爬梯子
// n=0, f(n)=1
// n=1, f(n)=1
// n=2, f(n)=2
// n=3, f(n)=3
// n=4, f(n)=5
// n=5, f(n)=8
//
// 可知:
// f(n)=f(n-1)+f(n-2)
#[allow(dead_code)]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut first = 0;
        let mut second = 0;
        let mut result = 1;
        (0..n).for_each(|_| {
            std::mem::swap(&mut first, &mut second);
            std::mem::swap(&mut second, &mut result);
            result = first + second;
        });
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(26), 196418);
        assert_eq!(Solution::climb_stairs(38), 63245986);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
