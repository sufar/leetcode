// https://leetcode.cn/problems/median-of-two-sorted-arrays/
//
// 给定两个大小分别为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的 中位数 。
//
// 算法的时间复杂度应该为 O(log (m+n)) 。
//
//  
//
// 示例 1：
//
// 输入：nums1 = [1,3], nums2 = [2]
// 输出：2.00000
// 解释：合并数组 = [1,2,3] ，中位数 2
// 示例 2：
//
// 输入：nums1 = [1,2], nums2 = [3,4]
// 输出：2.50000
// 解释：合并数组 = [1,2,3,4] ，中位数 (2 + 3) / 2 = 2.5
//  
//
//  
//
// 提示：
//
// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106

struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut p1, mut p2) = (0, 0);
        let (len1, len2) = (nums1.len(), nums2.len());
        let total = len1 + len2;
        if total % 2 == 0 {
            // 两个数字之和/2
            
        } else {
            // 只有中间一个数字
        }
        
        0.1
    }
}
