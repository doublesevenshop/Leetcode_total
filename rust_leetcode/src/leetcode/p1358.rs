pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s= s.as_bytes();
        let mut ans = 0;
        let mut left = 0;
        let mut cnt = [0; 3];
        for &c in s {
            cnt[(c-b'a') as usize] += 1;
            while cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 {
                cnt[(s[left]-b'a') as usize] -= 1;
                left += 1;
            }
            ans += left;
        }
        ans as _
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p1358() {
        // 在此编写测试代码
        assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
    }
}
