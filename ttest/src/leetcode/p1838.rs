pub struct Solution;


impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        // 与之前做过的题有些类似，不过还是以后要根据自己的思路来做题，不能看答案

        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut sum = 0;
        let mut ans = 1;

        for i in 0..n {
            sum += nums[i];
            if nums[i] * ans - sum > k {
                sum -= nums[i - ans as usize + 1];
            } else {
                ans += 1;
            }
        }
        ans -1
    }
}


#[cfg(test)]
mod test {
    use super::Solution;

    pub fn test_p1838() {
        assert_eq!(Solution::max_frequency(vec![1,4,8,13], 5), 2);
    }
}