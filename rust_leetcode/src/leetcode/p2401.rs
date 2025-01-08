pub struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut n = nums.len();

        let mut left = 0;
        let mut ans = 0;
        for right in 0..n {
            let mut cur = left;

            for i in left..right {
                if nums[i] & nums[right] != 0 {
                    cur = i+1;
                }
            }
            left = cur;
            ans = ans.max(right - left + 1);



        }
        ans as i32
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2401() {
        // 在此编写测试代码
    }
}
