#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::new();
        let mut prev = ' ';
        let mut count = 0;
        for c in s.chars()
        {
            if c == prev
            {
                count += 1;
            }
            else{
                count = 1;
                prev = c;
            }

            if count < 3
            {
                ans.push(c);
            }
        }
        ans
    }
}
fn main()
{

}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn case1()
    {
        assert_eq!(&Solution::make_fancy_string("leeetcode".to_string()),"leetcode");
    }
}