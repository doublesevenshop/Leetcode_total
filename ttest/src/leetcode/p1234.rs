pub struct Solution;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut cnt = vec![0; 26];
        let mut ans = n;
        let partial = (n/4) as i32;
        // 先统计出个数
        for c in s {
            cnt[(c-b'A') as usize] += 1;
        }

        if Self::already(&cnt, partial) {
            return 0;
        }
        let mut left = 0;

        for (right, &c) in s.iter().enumerate() {
            cnt[(c - b'A') as usize] -= 1;
            while Self::maybe(&cnt, partial) {
                ans = ans.min(right - left + 1);
                cnt[(s[left] - b'A') as usize] += 1;
                left += 1;
            }
        }

        ans as i32
    }
    fn already(cnt: &[i32], partial: i32) -> bool {
        for c in "QWER".as_bytes() {
            if cnt[(c-b'A') as usize] != partial {
                return false;
            }
        }
        true
    }
    fn maybe(cnt: &[i32], partial: i32) -> bool {
        for c in "QWER".as_bytes() {
            if cnt[(c-b'A') as usize] > partial {
                return false;
            }
        }
        true
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1234() {
        // 在此编写测试代码
        assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
    }
}
