struct Solution;
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let modulo = 1_000_000_007;
        let word_len = words[0].len();
        let target_len = target.len();

        let freq: Vec<[i32; 26]> = words.into_iter().fold(vec![[0; 26]; word_len], |mut m, w| {
            w.as_bytes()
                .iter()
                .enumerate()
                .for_each(|(i, &c)| m[i][(c - 'a' as u8) as usize] += 1);
            m
        });

        let mut dp = vec![vec![0_i64; target_len + 1]; word_len + 1];
        (0..=word_len).for_each(|word_idx| dp[word_idx][0] = 1);
        (1..=target_len)
            .flat_map(|i| (1..=word_len).map(move |j| (i, j)))
            .for_each(|(target_idx, word_idx)| {
                dp[word_idx][target_idx] = dp[word_idx - 1][target_idx];
                let c = (target.as_bytes()[target_idx - 1] - 'a' as u8) as usize;
                dp[word_idx][target_idx] +=
                    (dp[word_idx - 1][target_idx - 1] * freq[word_idx - 1][c] as i64) % modulo;
                dp[word_idx][target_idx] %= modulo;
            });
        dp[word_len][target_len] as i32
    }
}

// impl Solution {
//     pub fn num_ways(words: Vec<String>, target: String) -> i32 {
//         let word_length = words[0].len();
//         let target_length = target.len();

//         // Initialize DP table and frequency table
//         let mut dp =  vec![vec![-1; target_length]; word_length];
//         let mut char_frequency = vec![vec![0; 26]; word_length];

//         // Store the frequency of every character at each index in the words
//         for word in words.iter() {
//             for (j, character) in word.chars().enumerate() {
//                 let character = character as usize - 'a' as usize;
//                 char_frequency[j][character] += 1;
//             }
//         }

//         Self::get_words(&words, &target, 0, 0, &mut dp, &mut char_frequency)
//     }

//     fn get_words(
//         words : &Vec<String>,
//         target : &String,
//         words_index : usize,
//         target_index : usize,
//         dp : &mut Vec<Vec<i32>>,
//         char_frequency : &mut Vec<Vec<i32>>
//     ) -> i32 {
//         const MOD : i32 = 1_000_000_007;

//         // If the target is fully matched
//         if target_index == target.len()
//         {
//             return 1;
//         }

//         // If we have no more columns in the words or not enough columns left to match
//         // the target
//         if
//             words_index == words[0].len() ||
//             words[0].len() - words_index < target.len() - target_index
//         {
//             return 0;
//         }

//         // If already computed, return the stored result
//         if dp[words_index][target_index] != -1
//         {
//             return dp[words_index][target_index];
//         }

//         let mut count_ways = 0;
//         let cur_pos = target.chars().nth(target_index).unwrap() as usize - 'a' as usize;

//         // Don't match any character of the target with the current word column
//         count_ways += Self::get_words(
//             words,
//             target,
//             words_index + 1,
//             target_index,
//             dp,
//             char_frequency
//         );

//         // Match the current character of the target with the current word column
//         count_ways +=
//             char_frequency[words_index][cur_pos] *
//             Self::get_words(
//                 words,
//                 target,
//                 words_index + 1,
//                 target_index + 1,
//                 dp,
//                 char_frequency
//             );

//         // Store the result in dp and return the answer
//         dp[words_index][target_index] = count_ways % MOD;

//         return dp[words_index][target_index];
//     }
// }

fn main() {
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()],
            "aba".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["abba".to_string(), "baab".to_string(), "abab".to_string(), "baba".to_string()],
            "abba".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "a".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "aa".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "aaa".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "aaaa".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "aaaaa".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "aaaaaa".to_string()
        )
    );
    println!(
        "{:?}",
        Solution::num_ways(
            vec!["a".to_string(), "a".to_string(), "a".to_string(), "a".to_string()],
            "aaaaaaa".to_string()
        )
    );
}
