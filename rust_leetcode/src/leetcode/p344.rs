use std::mem::swap;


pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {

        let n = s.len();
        let (mut left, mut right) = (0, n-1);

        while left <= right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p344() {
        // 在此编写测试代码
        let mut s = vec!['a', 'b', 'c', 'd', 'e'];

        Solution::reverse_string(&mut s);
        println!("{:?}", s);
    }
}
