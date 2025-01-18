struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut vec = vec![0; 26];
        let mut res = 0;
        for c in s.chars() {
            vec[(c as u8 - 'a' as u8) as usize] += 1;
            res += 1;
            if vec[(c as u8 - 'a' as u8) as usize] == 3 
            {
                vec[(c as u8 - 'a' as u8) as usize] -= 2;
                res -= 2;
            }
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
    assert_eq!(Solution::minimum_length("a".to_string()), 1);
}







