use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut ans: i64 = 0;
        let mut left = 0;
        let mut num_max = *nums.iter().max().unwrap();
        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] == num_max {
                count += 1;
            }
            while count >= k {
                if nums[left] == num_max {
                    count -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }
        
        ans
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2962() {
        // 在此编写测试代码
        assert_eq!(Solution::count_subarrays(vec![1,3,2,3,3], 2),6);
    }
}
