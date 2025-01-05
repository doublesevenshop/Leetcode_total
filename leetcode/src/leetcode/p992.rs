pub struct Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let most = |k: i32| -> i32 {
            let mut freq = vec![0; n + 1];
            let mut count = 0;
            let mut sum = 0;
            let mut lo = 0;
            for hi in 0..n {
                if freq[a[hi] as usize] == 0 { count += 1; }
                freq[a[hi] as usize] += 1;
                while count > k {
                    freq[a[lo] as usize] -= 1;
                    if freq[a[lo] as usize] == 0 { count -= 1; }
                    lo += 1;
                }
                sum += hi - lo + 1;
            }
            sum as i32
        };

        most(k) - most(k - 1)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p992() {
        // 在此编写测试代码
        assert_eq!(Solution::subarrays_with_k_distinct(vec![1,2,1,2,3], 2),7);
    }
}
