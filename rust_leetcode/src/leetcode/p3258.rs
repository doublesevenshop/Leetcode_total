pub struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut cnt = [0; 2];
        let (mut left, mut ans) = (0, 0);
        let s = s.as_bytes();

        for right in 0..s.len() {
            cnt[(s[right] % 2) as usize] += 1;
            while cnt[0] > k && cnt[1] > k {
                cnt[(s[left] % 2) as usize] -= 1;
                left += 1;
            }
            ans += (right - left + 1) as i32;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p3258() {
        // 在此编写测试代码
        assert_eq!(Solution::count_k_constraint_substrings("10101".to_string(), 1), 12);
    }
}
