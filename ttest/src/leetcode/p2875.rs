pub struct Solution;

impl Solution {
    pub fn min_size_subarray(nums: Vec<i32>, target: i32) -> i32 {
        let total: i64 = nums.iter().map(|&x|x as i64).sum();
        let n = nums.len();
        let mut ans = usize::MAX;

        let mut left = 0;
        let mut sum = 0;
        for right in 0..=n*2 {
            sum += nums[right%n];

            while sum > (target as i64 % total) as i32 {
                sum -= nums[left%n];
                left += 1;
            }
            if sum == (target as i64 % total) as i32 {
                ans = ans.min(right - left + 1);
            }
        }

        if ans < usize::MAX {
            ans as i32 + (target as i64 / total) as i32 * n as i32
        } else {
            -1
        }
        
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2875() {
        // 在此编写测试代码
        assert_eq!(Solution::min_size_subarray(vec![1,1,1], 1000000000), 1000000000);
    }
}
