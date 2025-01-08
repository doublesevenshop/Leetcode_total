pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.as_bytes();
        let s: Vec<_> = s
            .iter()
            .map(|&x|x.to_ascii_uppercase())
            .filter(|&x| x.is_ascii_alphanumeric())
            .collect();

            if s.len() == 0 {
            return true;
        }
        let (mut left, mut right) = (0, s.len()-1);
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
        
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p125() {
        // 在此编写测试代码
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
    }
}
