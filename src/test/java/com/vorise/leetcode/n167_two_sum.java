package com.vorise.leetcode;

/**
 * https://leetcode.cn/problems/two-sum-ii-input-array-is-sorted/description/
 *
 * 167. 两数之和 II - 输入有序数组
 *
 * 给你一个下标从 1 开始的整数数组 numbers ，该数组已按 非递减顺序排列 ，请你从数组中找出满足相加之和等于目标数 target
 * 的两个数。如果设这两个数分别是 numbers[index1] 和 numbers[index2] ，则 1 <= index1 < index2 <=
 * numbers.length 。
 * 
 * 以长度为 2 的整数数组 [index1, index2] 的形式返回这两个整数的下标 index1 和 index2。
 * 
 * 你可以假设每个输入 只对应唯一的答案 ，而且你 不可以 重复使用相同的元素。
 * 
 * 你所设计的解决方案必须只使用常量级的额外空间。
 * 
 * 
 * 示例 1：
 * 
 * 输入：numbers = [2,7,11,15], target = 9
 * 输出：[1,2]
 * 解释：2 与 7 之和等于目标数 9 。因此 index1 = 1, index2 = 2 。返回 [1, 2] 。
 * 示例 2：
 * 
 * 输入：numbers = [2,3,4], target = 6
 * 输出：[1,3]
 * 解释：2 与 4 之和等于目标数 6 。因此 index1 = 1, index2 = 3 。返回 [1, 3] 。
 * 示例 3：
 * 
 * 输入：numbers = [-1,0], target = -1
 * 输出：[1,2]
 * 解释：-1 与 0 之和等于目标数 -1 。因此 index1 = 1, index2 = 2 。返回 [1, 2] 。
 * 
 * 
 * 提示：
 * 
 * 2 <= numbers.length <= 3 * 104
 * -1000 <= numbers[i] <= 1000
 * numbers 按 非递减顺序 排列
 * -1000 <= target <= 1000
 * 仅存在一个有效答案
 * 
 * @author zugle on 2023-08-23
 */
public class n167_two_sum {

    class Solution {
        /**
         * 使用双指针，一个指针指向值较小的元素，一个指针指向值较大的元素。指向较小元素的指针从头向尾遍历，指向较大元素的指针从尾向头遍历。
         * 
         * 如果两个指针指向元素的和 sum == target，那么得到要求的结果；
         * 如果 sum > target，移动较大的元素，使 sum 变小一些；
         * 如果 sum < target，移动较小的元素，使 sum 变大一些。
         * 数组中的元素最多遍历一次，时间复杂度为 O(N)。只使用了两个额外变量，空间复杂度为 O(1)。
         */
        public int[] twoSum(int[] numbers, int target) {

            int left = 0;
            int right = numbers.length - 1;

            while (left != right) {
                int result = numbers[left] + numbers[right];
                if (result == target) {
                    return new int[] { left + 1, right + 1 };
                } else if (result > target) {
                    right--;
                } else {
                    left++;
                }
            }

            return new int[] {};
        }
    }
}
