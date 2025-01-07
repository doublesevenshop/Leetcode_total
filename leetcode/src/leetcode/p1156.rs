pub struct Solution;

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let (mut left, mut middle, mut right, mut ans) = (0, 0, 0, 0);
        let text = text.as_bytes();
        let n = text.len();

        let mut freq = vec![0; 26];

        for &c in text {
            freq[(c-b'a') as usize] += 1;
        }

        while left < n {
            middle = left;
            
            // 先算出middle左边的长度
            while middle < n && text[left] == text[middle] {
                middle += 1;
            }
            // 实际计算为：[left, middle-1]  length = middle - 1 - left + 1 = middle - left
            let length_left = middle - left;
            // 右边是middle再往右的一个
            right = middle + 1;
            // 计算出middle右边的长度
            while right < n && text[right] == text[left] {
                right += 1;
            }
            // 实际计算为：[middle+1, right - 1] length = right - 1 - middle - 1 + 1 = right - middle - 1
            let length_right = right - middle - 1;
            // 由于可以交换一个，但是交换又不能长过总数，所以要找到拼接和总数之间较小的那个
            ans = ans.max(usize::min(length_left+length_right+1, freq[(text[left]-b'a') as usize]));
            // 迭代left
            left = middle;
        }
        ans as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p1156() {
        // 在此编写测试代码
        assert_eq!(Solution::max_rep_opt1("ababa".to_string()), 3);
    }
}
