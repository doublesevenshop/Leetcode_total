pub struct Solution;

impl Solution {
    pub fn num_friend_requests(mut ages: Vec<i32>) -> i32 {
        ages.sort();
        let (mut left, mut right, mut ans) = (0, 0, 0);

        for &c in &ages {
            if c < 15 {
                continue;
            }
            while ages[left] as f64 <= 0.5 * c as f64 + 7f64 {
                left += 1;
            }
            while right + 1 < ages.len() && ages[right+1] <= c {
                right += 1;
            }
            ans += (right - left);
        }
        
        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p825() {
        // 在此编写测试代码
        assert_eq!(Solution::num_friend_requests(vec![16,16]),2);
    }
}
