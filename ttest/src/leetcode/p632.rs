use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut hash = BinaryHeap::with_capacity(nums.len());
        let mut r= i32::MIN;
        for (i, arr) in nums.iter().enumerate() {
            hash.push((-arr[0], i, 0));
            r = r.max(arr[0]);
        }

        let mut ans_l = -hash.peek().unwrap().0;
        let mut ans_r = r;
        while hash.peek().unwrap().2 + 1 < nums[hash.peek().unwrap().1].len() {
            let(_,i, j) = hash.pop().unwrap();
            let x= nums[i][j+1];
            hash.push((-x, i, j+1));
            r = r.max(x);
            let l = -hash.peek().unwrap().0;
            if r - l < ans_r - ans_l {
                ans_l = l;
                ans_r = r;
            }
        }
        vec![ans_l, ans_r]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p632() {
        // 在此编写测试代码
    }
}
