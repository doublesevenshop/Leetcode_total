pub struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let s = word1.as_bytes();
        let t = word2.as_bytes();
        let (slen, tlen) = (s.len(), t.len());
        if slen < tlen {
            return 0;
        }
        let mut cnt = vec![0; 26];

        for &e in t {
            cnt[(e-b'a') as usize] += 1;
        }
        let mut less = 0;

        for &c in &cnt {
            if c > 0 {
                less += 1;
            }
        }

        let mut ans: i64 = 0;
        let mut left = 0;

        for &c in s {
            cnt[(c-b'a') as usize] -= 1;
            if cnt[(c-b'a') as usize] == 0 {
                less -= 1;
            }

            while less == 0 {
                let out_char = s[left];
                left += 1;

                if cnt[(out_char - b'a') as usize] == 0 {
                    less += 1;
                }
                cnt[(out_char - b'a') as usize] += 1;
            }
            ans += left as i64;
        }
        
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p3298() {
        // 在此编写测试代码
        assert_eq!(Solution::valid_substring_count("bcca".to_string(), "abc".to_string()), 1);
    }
}
