pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String, k: i32) -> i32 {
        let mut cnt = vec![0; 26];
        let s = s.as_bytes();
        let mut left = 0;
        let mut ans = 0;

        for right in 0..s.len() {
            cnt[(s[right]-b'a') as usize] += 1;

            while cnt[(s[right]-b'a') as usize]  >= k {
                cnt[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
            ans += left;
        }

        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p3325() {
        // 在此编写测试代码
        assert_eq!(Solution::number_of_substrings("abacb".to_string(), 2),4);
    }
}
