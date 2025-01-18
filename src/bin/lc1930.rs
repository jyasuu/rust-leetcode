struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut res = 0;
        let chars = s.as_bytes();
        let mut start = vec![s.len(); 26];
        let mut end = vec![0; 26];
        let mut sets = vec![std::collections::HashSet::new(); 26];
        
        
        for i in 0..s.len()
        {
            let idx = (chars[i] - b'a') as usize;
            start[idx] = i.min(start[idx]);
            end[idx] = i;
        }
        
        for i in 0..s.len()
        {
            for j in 0..26
            {
                if start[j] < end[j] && i > start[j] && i < end[j]
                {
                    sets[j].insert(chars[i]);
                }
            }
        }
        for i in 0..26
        {
            res += sets[i].len() as i32;
        }

        res  
    }
}

fn main()
{
    println!("{:?}", Solution::count_palindromic_subsequence("aabca".to_string()));
}



//   a a b c a
// a 1 1 0 0 1
// c 0 0 0 1 0
// b 0 0 1 0 0
// a 1 1 0 0 1
// a 1 1 0 0 1 