pub struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let mut ans: String = String::new();

        let mut left = 0;
        let mut count = 0;
        let s = s.as_bytes();
        let mut min_len = usize::MAX;

        for (right, &c) in s.iter().enumerate() {
            if c == b'1' {
                count += 1;
            }

            while count >= k {
                if count == k {
                    let len = right - left + 1;
                    let sub_str = String::from_utf8(s[left..=right].to_vec()).unwrap();
                    if len < min_len || (len == min_len && sub_str < ans) {
                        min_len = len;
                        ans = sub_str;
                    }
                }
                if s[left] == b'1' {
                    count -= 1;
                }
                left += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2904() {
        let s = "100011001".to_string();
        let k = 3;
        let out = "11001".to_string();
        // 在此编写测试代码
        assert_eq!(Solution::shortest_beautiful_substring(s, k), out);
    }
}
