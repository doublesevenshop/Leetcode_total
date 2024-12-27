pub struct Solution;

impl Solution {
    pub fn longest_equal_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut pos_list = vec![vec![]; nums.len()+1];
        for (i, &x) in nums.iter().enumerate() {
            let pos = &mut pos_list[x as usize];
            pos.push(i - pos.len());
        }
        let mut ans = 0;

        for pos in pos_list {
            let mut left = 0;
            for (right, &p) in pos.iter().enumerate() {
                while p - pos[left] > k as usize {
                    left += 1;
                }
                ans = ans.max(right - left + 1);
            }
        }
       
        ans as _
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test_p2831() {
        // 在此编写测试代码
    }
}
