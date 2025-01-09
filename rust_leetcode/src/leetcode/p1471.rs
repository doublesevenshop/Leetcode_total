pub struct Solution;

impl Solution {
    pub fn get_strongest(mut arr: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, arr.len() - 1);
        arr.sort(); // 先对数组进行排序
        let m = arr[(arr.len() - 1) / 2]; // 中位数
        let n = arr.len();
        let mut ans = vec![];

        let mut a = i32::abs(arr[left] - m);
        let mut b = i32::abs(arr[right] - m);

        // 选择 k 个最强的元素
        while right - left > (n - k as usize) - 1 {
            println!("a: {a}, b:{b}");
            if a > b {
                ans.push(arr[left]);
                left += 1;
                a = i32::abs(arr[left] - m);
            } else if a < b {
                ans.push(arr[right]);
                right -= 1; // 这里应该是 right 减少
                b = i32::abs(arr[right] - m);
            } else {
                if arr[left] >= arr[right] {
                    ans.push(arr[left]);
                    left += 1;
                    a = i32::abs(arr[left] - m);
                } else {
                    ans.push(arr[right]);
                    right -= 1; // 这里应该是 right 减少
                    b = i32::abs(arr[right] - m);
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
    fn test_p1471() {
        assert_eq!(Solution::get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
    }
}
