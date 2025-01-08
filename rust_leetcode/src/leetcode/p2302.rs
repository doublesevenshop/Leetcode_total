pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        fn score(nums: &Vec<i32>) -> i64 {
            let sum: i64 = nums.iter().cloned().map(|x|x as i64).sum();
            let score: i64 = sum * nums.len() as i64;
            score
        }
        
        
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p2302() {
        // 在此编写测试代码
        
    }
}
