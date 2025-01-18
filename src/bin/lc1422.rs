struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut chars = s.chars();
        let n = s.len();
        let mut ones = 0;
        let mut zeros = 0;
        while let Some(c) = chars.next()
        {
            match c
            {
                '0' => {
                },
                _ => {
                    ones += 1;
                },
            }
        }
        let mut chars = s.chars().take(n-1);
        let mut res = 0;
        while let Some(c) = chars.next()
        {
            match c
            {
                '0' => {
                    zeros += 1;
                },
                _ => {
                    ones -= 1;
                },
            }
            res = res.max( zeros + ones);
        }
        res
    }
}

fn main() {
    println!("{:?}", Solution::max_score("011101".to_string()));
    println!("{:?}", Solution::max_score("00111".to_string()));
    println!("{:?}", Solution::max_score("1111".to_string()));
    println!("{:?}", Solution::max_score("00".to_string()));
    println!("{:?}", Solution::max_score("00".to_string()));
    println!("{:?}", Solution::max_score("010101".to_string()));
    println!("{:?}", Solution::max_score("0101010".to_string()));
    println!("{:?}", Solution::max_score("01010101".to_string()));
    println!("{:?}", Solution::max_score("010101010".to_string()));
    println!("{:?}", Solution::max_score("0101010101".to_string()));
    println!("{:?}", Solution::max_score("01010101010".to_string()));
    println!("{:?}", Solution::max_score("010101010101".to_string()));
    println!("{:?}", Solution::max_score("0101010101010".to_string()));
    println!("{:?}", Solution::max_score("01010101010101".to_string()));
    println!("{:?}", Solution::max_score("010101010101010".to_string()));
    println!("{:?}", Solution::max_score("0101010101010101".to_string()));
    println!("{:?}", Solution::max_score("01010101010101010".to_string()));
    println!("{:?}", Solution::max_score("010101010101010101".to_string()));
    println!("{:?}", Solution::max_score("0101010101010101010".to_string()));
    println!("{:?}", Solution::max_score("01010101010101010101".to_string()));
    println!("{:?}", Solution::max_score("010101010101010101010".to_string()));
    println!("{:?}", Solution::max_score("0101010101010101010101".to_string()));
    println!("{:?}", Solution::max_score("01010101010101010101010".to_string()));
    println!("{:?}", Solution::max_score("010101010101010101010101".to_string()));
    println!("{:?}", Solution::max_score("0101010101010101010101010".to_string()));
    println!("{:?}", Solution::max_score("01010101010101010101010101".to_string()));
}