
pub struct Solution;

impl Solution {
    pub fn min_window(S: String, t: String) -> String {
        fn is_covered(cnt_s: &[i32; 128], cnt_t: &[i32; 128]) -> bool {
            for i in b'A'..=b'Z' {
                if cnt_s[i as usize] < cnt_t[i as usize] {
                    return false;
                }
            }
            for i in b'a'..=b'z' {
                if cnt_s[i as usize] < cnt_t[i as usize] {
                    return false;
                }
            }
            
            true
        }

        let s = S.as_bytes();
        let n = s.len();
        let mut ans_left = 0;
        let mut ans_right = n;
        let mut cnt_s = [0; 128];
        let mut cnt_t = [0; 128];
        
        for c in t.bytes() {
            cnt_t[c as usize] += 1;
        }
        let mut left = 0;
        for (right, &c) in s.iter().enumerate() {
            cnt_s[c as usize] += 1;

            while is_covered(&cnt_s, &cnt_t) {
                if right - left < ans_right - ans_left {
                    ans_left = left;
                    ans_right = right;
                }
                cnt_s[s[left] as usize] -= 1;
                left += 1;
            }
        }
        if ans_right < n {
            return S[ans_left..=ans_right].to_string();
        } else {
            return String::new();
        }
        unreachable!()

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p76() {
        // 在此编写测试代码
        let S = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(Solution::min_window(S, t), "BANC".to_string());
    }
}
