struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut result = 0;
        for i in 0..words.len() {
            for j in i..words.len() {
                if i != j && words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main ()
{
    let words = vec!["abcd".to_string(),"dcba".to_string(),"lls".to_string(),"s".to_string(),"sssll".to_string()];
    let result = Solution::count_prefix_suffix_pairs(words);
    println!("{:?}", result);
}