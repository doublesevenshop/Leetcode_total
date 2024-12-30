pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }
        let (mut left, mut count, mut mul) = (0, 0, 1);

        for right in 0..nums.len() {
            mul *= nums[right];
            while mul >= k {
                mul /= nums[left];
                left += 1;
            }
            count += (right - left + 1) as i32;
        }
        
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p713() {
        // 在此编写测试代码
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![10,5,2,6], 100),8);
        assert_eq!(Solution::num_subarray_product_less_than_k(vec![1,2,3], 0),0);
    }
}
