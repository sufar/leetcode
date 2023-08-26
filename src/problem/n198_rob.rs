struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 动态规划，抢了i-1，就不能抢i了
        // d[i]=max(d[i-2]+nums[i], d[i-1])
        let mut pre2 = 0;
        let mut pre1 = 0;
        for (_, v) in nums.iter().enumerate() {
            let cur = pre1.max(pre2 + v);
            pre2 = pre1;
            pre1 = cur;
        }
        return pre1;
    }
}
