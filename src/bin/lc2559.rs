struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let vowel_words = words.iter().map(|word| {
            (word.chars().next().unwrap() == 'a' || word.chars().next().unwrap() == 'e' || word.chars().next().unwrap() == 'i' || word.chars().next().unwrap() == 'o' || word.chars().next().unwrap() == 'u' ) &&
            (word.chars().last().unwrap() == 'a' || word.chars().last().unwrap() == 'e' || word.chars().last().unwrap() == 'i' || word.chars().last().unwrap() == 'o' || word.chars().last().unwrap() == 'u')
        }).collect::<Vec<bool>>();
        let mut count = 0;
        let mut count_vowel = Vec::new();
        for i in 0..vowel_words.len() {
            if vowel_words[i] {
                count += 1;
            }
            count_vowel.push(count);
        }   
        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize;
            ans.push(count_vowel[r] - count_vowel[l] + if vowel_words[l] {1} else {0});
        }
        ans
        
    }
}

fn main()
{
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    println!("{:?}", Solution::vowel_strings(vec!["aeiou".to_string(), "aa".to_string(), "a".to_string(), "o".to_string()], vec![vec![0,2], vec![1,2], vec![0,3], vec![0,3]]));
    
}