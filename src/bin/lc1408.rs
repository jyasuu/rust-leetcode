struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        for i in 0..words.len() {
            for j in 0..words.len() {
                if i != j && words[j].contains(&words[i]) {
                    result.push(words[i].clone());
                    break;
                }
            }
        }
        result
    }
}

fn main ()
{
    let words = vec!["mass".to_string(),"as".to_string(),"hero".to_string(),"superhero".to_string()];
    let result = Solution::string_matching(words);
    println!("{:?}", result);
}