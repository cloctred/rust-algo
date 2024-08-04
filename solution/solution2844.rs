/// 2844.生成特殊数字的最少操作
/// minimum-operations-to-make-a-special-number
/// <p>给你一个下标从 <strong>0</strong> 开始的字符串 <code>num</code> ，表示一个非负整数。</p>
///
/// <p>在一次操作中，您可以选择 <code>num</code> 的任意一位数字并将其删除。请注意，如果你删除 <code>num</code> 中的所有数字，则 <code>num</code> 变为 <code>0</code>。</p>
///
/// <p>返回最少需要多少次操作可以使 <code>num</code> 变成特殊数字。</p>
///
/// <p>如果整数 <code>x</code> 能被 <code>25</code> 整除，则该整数 <code>x</code> 被认为是特殊数字。</p>
///
/// <p>&nbsp;</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>num = "2245047"
/// <strong>输出：</strong>2
/// <strong>解释：</strong>删除数字 num[5] 和 num[6] ，得到数字 "22450" ，可以被 25 整除。
/// 可以证明要使数字变成特殊数字，最少需要删除 2 位数字。</pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>num = "2908305"
/// <strong>输出：</strong>3
/// <strong>解释：</strong>删除 num[3]、num[4] 和 num[6] ，得到数字 "2900" ，可以被 25 整除。
/// 可以证明要使数字变成特殊数字，最少需要删除 3 位数字。</pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>num = "10"
/// <strong>输出：</strong>1
/// <strong>解释：</strong>删除 num[0] ，得到数字 "0" ，可以被 25 整除。
/// 可以证明要使数字变成特殊数字，最少需要删除 1 位数字。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= num.length &lt;= 100</code></li>
/// 	<li><code>num</code> 仅由数字 <code>'0'</code> 到 <code>'9'</code> 组成</li>
/// 	<li><code>num</code> 不含任何前导零</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/minimum-operations-to-make-a-special-number/">生成特殊数字的最少操作</a>
pub struct Solution;

impl Solution {
    pub fn minimum_operations(num: String) -> i32 {
        let n = num.len() as i32;
        let mut found0 = false;
        let mut found5 = false;
        for (i, c) in num.bytes().enumerate().rev() {
            if found0 && (c == b'0' || c == b'5') || found5 && (c == b'2' || c == b'7') {
                return n - i as i32 - 2;
            }
            if c == b'0' {
                found0 = true;
            } else if c == b'5' {
                found5 = true;
            }
        }
        if found0 {
            n - 1
        } else {
            n
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::minimum_operations("2245047".to_string()), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::minimum_operations("225".to_string()), 0)
    }
}
