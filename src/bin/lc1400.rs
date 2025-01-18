struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        let mut odd = 0;
        for i in 0..26 {
            if count[i] % 2 == 1 {
                odd += 1;
            }
        }
        odd <= k && s.len() >= k as usize
        
    }
}

fn main() {
    assert_eq!(Solution::can_construct("annabelle".to_string(), 2), true);
    assert_eq!(Solution::can_construct("leetcode".to_string(), 3), false);
    assert_eq!(Solution::can_construct("true".to_string(), 4), true);
    assert_eq!(Solution::can_construct("yzyzyzyzyzyzyzy".to_string(), 2), true);
    assert_eq!(Solution::can_construct("cr".to_string(), 7), false);
}
