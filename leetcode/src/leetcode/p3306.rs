use std::alloc::GlobalAlloc;

pub struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let word = word.as_bytes();
        let word_size = word.len();
        
        let check = |c: u8| -> bool {
            matches!(c, b'a' | b'e' | b'i'| b'o' | b'u')
        };

        let calc = |goal: i32| -> i64 {
            let mut cnt = vec![0; 26];
            let mut yuan_num = 0;
            let mut fu_num = 0;
            let mut ans: i64 = 0;
            let mut left = 0;

            for right in 0..word_size {
                let pos = (word[right] - b'a') as usize;
                if check(word[right]) {
                    let t = cnt[pos];
                    cnt[pos] += 1;
                    if t == 0 {
                        yuan_num += 1;
                    }
                } else {
                    fu_num += 1;
                }

                while yuan_num == 5 && fu_num >= goal {
                    let out_pos = (word[left] - b'a') as usize;
                    if check(word[left]) {
                        cnt[out_pos] -= 1;
                        
                        if cnt[out_pos] == 0 {
                            yuan_num -= 1;
                        }
                    } else {
                        fu_num -= 1;
                    }
                    left += 1;
                }
                ans += left as i64;
            }
            ans
        };
        calc(k) - calc(k+1)

       

    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_p3306() {
        // 在此编写测试代码
        assert_eq!(Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
        assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);

    }
}
