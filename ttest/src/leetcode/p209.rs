pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut ans = i32::MAX;
        let mut left = 0;

        for (right, &c) in nums.iter().enumerate() {
            sum += c;

            while sum >= target {
                ans  = ans.min((right - left + 1)as i32);
                sum -= nums[left];
                left += 1;
            }

        }
        if ans != i32::MAX {
            return ans;
        } else {
            return 0;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p209() {
        // 在此编写测试代码
        assert_eq!
        (Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]),2);
    }
}
