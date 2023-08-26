struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 动态规划，环形n-2,n-1比较
        let size = nums.len();
        if size == 0 {
            return 0;
        }
        if size == 1 {
            return nums[0];
        }

        let s1 = Solution::rob2(&nums, 0, size - 2);
        let s2 = Solution::rob2(&nums, 1, size - 1);
        return s1.max(s2);

    }

    pub fn rob2(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
        let mut pre1=0;
        let mut pre2=0;
        for (i, v) in  nums.iter().enumerate() {
            if i < left {
                continue;
            }
            if i > right {
                break;
            }

            let cur = pre1.max(pre2 + v);
            pre2 = pre1;
            pre1 = cur;
        }
        return pre1;
    }
}
