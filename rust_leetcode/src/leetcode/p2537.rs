use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        // 初始化变量
        let mut result: i64 = 0; // 最终答案
        let mut count_map: HashMap<i32, i32> = HashMap::new(); // 用于存储每个元素的出现次数
        let mut pairs = 0; // 当前窗口内的有效对数
        let mut left = 0; // 滑动窗口左边界

        // 遍历数组
        for &num in &nums {
            // 更新当前窗口中与 num 可以形成的有效对数
            pairs += count_map.get(&num).unwrap_or(&0);

            // 更新当前元素的计数
            *count_map.entry(num).or_insert(0) += 1;

            // 确保窗口中的有效对数不超过 k
            while pairs >= k {
                let left_num = nums[left];
                *count_map.get_mut(&left_num).unwrap() -= 1; // 移除左边元素
                pairs -= count_map.get(&left_num).unwrap_or(&0); // 更新有效对数
                left += 1; // 移动左指针
            }

            // 累计答案（当前窗口左边界位置）
            result += left as i64;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2537() {
        // 测试用例
        assert_eq!(Solution::count_good(vec![1, 1, 1, 1, 1], 10), 1);
    }
}
