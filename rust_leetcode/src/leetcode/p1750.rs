pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();

        let (mut left, mut right) = (0, s.len()-1);
        
        while left < right && s[left] == s[right] {
            let target = s[left];

            while left <= right && s[left] == target {
                left += 1;
            }
            while left <= right && s[right] == target {
                right -= 1;
            }

        }
        (right + 1 - left) as _
        
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p1750() {
        // 在此编写测试代码
        assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    }
}
