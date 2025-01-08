pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut res = vec![];
        let mut p_count = vec![0; 26];
        let mut s_count = vec![0; 26];
        for &c in p.iter() {
            p_count[(c - b'a') as usize] += 1;
        }
        let n = s.len();
        let m = p.len();
        for i in 0..n {
            s_count[(s[i] - b'a') as usize] += 1;
            if i >= m {
                s_count[(s[i - m] - b'a') as usize] -= 1;
            }
            if s_count == p_count {
                res.push((i + 1 - m) as i32);
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p438() {
        // 在此编写测试代码
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let res = Solution::find_anagrams(s, p);
        assert_eq!(res, vec![0, 6]);
    }
}
