struct Solution;



impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0;
        for word in words {
            if word.starts_with(&pref) {
                count += 1;
            }
        }
        count
    }
}



fn main()
{
    let words = vec!["dog".to_string(), "cat".to_string(), "dove".to_string(), "duck".to_string()];
    let pref = "do".to_string();
    let count = Solution::prefix_count(words, pref);
    println!("count: {}", count);
}
