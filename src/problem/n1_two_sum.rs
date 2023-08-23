use std::collections::HashMap;

// https://leetcode.cn/problems/two-sum/
//
// 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
//
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
//
// 你可以按任意顺序返回答案。
//
//  
//
// 示例 1：
//
// 输入：nums = [2,7,11,15], target = 9
// 输出：[0,1]
// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
// 示例 2：
//
// 输入：nums = [3,2,4], target = 6
// 输出：[1,2]
// 示例 3：
//
// 输入：nums = [3,3], target = 6
// 输出：[0,1]
//  
//
// 提示：
//
// 2 <= nums.length <= 104
// -109 <= nums[i] <= 109
// -109 <= target <= 109
// 只会存在一个有效答案
//  
//
// 进阶：你可以想出一个时间复杂度小于 O(n2) 的算法吗？
//
struct Solution{}

impl Solution {

    // 暴力解法：复杂度为O(n^2)
    pub fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() - 1 {
            for j in i+1..nums.len() {
                if nums.get(i).unwrap() + nums.get(j).unwrap() == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }

    // hash表，时间复杂度：O(n)，空间复杂度：O(n)
    pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&x) = map.get(&(target - num)) {
                return vec![i as i32, x];
            }
            map.insert(num, i as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum1() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let result = Solution::two_sum1(nums, target);
        println!("{:?}", result);
    }

    #[test]
    fn test_two_sum2() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let result = Solution::two_sum2(nums, target);
        println!("{:?}", result);
    }
}
