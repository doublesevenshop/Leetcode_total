pub struct Solution;

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let (mut left, mut right) = (0, n - 1);
        let mut ans = n as i32; // 初始化为最大长度

        // 找到从左边开始的非递减子数组
        while left + 1 < n && arr[left] <= arr[left + 1] {
            left += 1;
        }

        // 找到从右边开始的非递减子数组
        while right > 0 && arr[right - 1] <= arr[right] {
            right -= 1;
        }

        // 如果整个数组本身已经是非递减的
        if left >= right {
            return 0;
        }

        // 初始最小子数组长度是从左边或右边切割
        ans = std::cmp::min(n as i32 - 1 - left as i32, right as i32);

        // 逐步查找通过删除一个子数组的最小长度
        for l in 0..=left {
            for r in right..n {
                if arr[l] <= arr[r] {
                    ans = std::cmp::min(ans, (r - l - 1) as i32);
                    break; // 找到第一个符合条件的对，退出内层循环
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_p1574() {
        // 在此编写测试代码
        let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
        let output = 3;

        assert_eq!(Solution::find_length_of_shortest_subarray(arr), output);
    }
}