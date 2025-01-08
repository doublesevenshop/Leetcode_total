use std::collections::{BTreeMap, BTreeSet};

pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut freq = BTreeMap::new();
        let n = nums.len();
        let (mut left, mut right) = (0, 0);
        let mut ans = 0;
        while right < n {
            *freq.entry(nums[right]).or_insert(0) += 1;

            while freq.iter().next_back().unwrap().0 - freq.iter().next().unwrap().0 > limit {
                let count = freq.get_mut(&nums[left]).unwrap();

                *count -= 1;

                if *count == 0 {
                    freq.remove(&nums[left]);
                }
                left += 1;
            }
            ans = ans.max(right - left + 1);
            right += 1;
        }

        ans as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p1438() {
        // 在此编写测试代码
        assert_eq!(Solution::longest_subarray(vec![8,2,4,7], 4), 2);
    }
}
