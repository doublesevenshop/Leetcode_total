use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut window: HashMap<i32, i32> = HashMap::new();
        let (mut ans, mut left) = (0, 0);

        // 统计数组中有多少种不同的数字
        for &num in &nums {
            *hash.entry(num).or_insert(0) += 1;
        }
        let total_distinct = hash.len();

        // 使用右指针扩展窗口
        for right in 0..nums.len() {
            *window.entry(nums[right]).or_insert(0) += 1;

            // 当窗口内的不同数字数目达到 total_distinct，移动左指针缩小窗口
            while window.len() == total_distinct {
                // 移动左指针，更新窗口
                let left_num = nums[left];
                *window.entry(left_num).or_insert(0) -= 1;
                if window[&left_num] == 0 {
                    window.remove(&left_num);
                }
                left += 1;
            }
            ans += left
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2799() {
        // 测试
        assert_eq!(Solution::count_complete_subarrays(vec![1, 3, 1, 2, 2]), 4);
    }
}
