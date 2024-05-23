/// 2831.找出最长等值子数组
///
/// <p>给你一个下标从 <strong>0</strong> 开始的整数数组 <code>nums</code> 和一个整数 <code>k</code> 。</p>
///
/// <p>如果子数组中所有元素都相等，则认为子数组是一个 <strong>等值子数组</strong> 。注意，空数组是 <strong>等值子数组</strong> 。</p>
///
/// <p>从 <code>nums</code> 中删除最多 <code>k</code> 个元素后，返回可能的最长等值子数组的长度。</p>
///
/// <p><strong>子数组</strong> 是数组中一个连续且可能为空的元素序列。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [1,3,2,3,1,3], k = 3
/// <strong>输出：</strong>3
/// <strong>解释：</strong>最优的方案是删除下标 2 和下标 4 的元素。
/// 删除后，nums 等于 [1, 3, 3, 3] 。
/// 最长等值子数组从 i = 1 开始到 j = 3 结束，长度等于 3 。
/// 可以证明无法创建更长的等值子数组。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [1,1,2,2,1,1], k = 2
/// <strong>输出：</strong>4
/// <strong>解释：</strong>最优的方案是删除下标 2 和下标 3 的元素。
/// 删除后，nums 等于 [1, 1, 1, 1] 。
/// 数组自身就是等值子数组，长度等于 4 。
/// 可以证明无法创建更长的等值子数组。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= nums.length</code></li>
/// 	<li><code>0 &lt;= k &lt;= nums.length</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-the-longest-equal-subarray/">找出最长等值子数组</a>
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let entry = map.entry(value).or_insert(vec![]);
            entry.push(index as i32);
        }

        let mut ans = 0;

        for v in map.values() {
            let mut left = 0;
            for right in 0..v.len() {
                let mut same_length = right - left + 1;
                let mut real_length = v[right] - v[left] + 1;
                while real_length - same_length as i32 > k {
                    left += 1;
                    same_length = right - left + 1;
                    real_length = v[right] - v[left] + 1;
                }
                ans = ans.max(right - left + 1);
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_1() {
        assert_eq!(
            Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2),
            4
        );

        assert_eq!(Solution::longest_equal_subarray(vec![1, 2, 1], 0), 1);
    }
}
