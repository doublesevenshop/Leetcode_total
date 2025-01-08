use std::sync::atomic::fence;

pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut freq = vec![0; 26];
        let (mut left, mut right, mut ans) = (0, 0, 0);
        let mut mmax = 0;
        let s=  s.as_bytes();
        let n = s.len();

        while right < n {
            freq[(s[right]-b'A') as usize] += 1;
            mmax = mmax.max(freq[(s[right]-b'A') as usize]);
            if (right - left + 1 - mmax > k as usize) {
                freq[(s[left] - b'A') as usize] -= 1;
                left += 1;
            }
            right += 1;
        }

        (right - left) as i32

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p424() {
        // 在此编写测试代码
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    }
}
