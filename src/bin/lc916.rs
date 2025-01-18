struct Solution;


impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut res = vec![];
        let mut b = vec![0; 26];
        for word in words2.iter() {
            let mut b2 = vec![0; 26];
            for c in word.chars() {
                b2[c as usize - 'a' as usize] += 1;
            }
            for i in 0..26 {
                b[i] = b[i].max(b2[i]);
            }
        }
        //println!("{:?}", b);
        for word in words1.iter() {
            let mut a = vec![0; 26];
            for c in word.chars() {
                a[c as usize - 'a' as usize] += 1;
            }
            //println!("{:?} {:?}",word, a);
            let mut flag = true;
            for i in 0..26 {
                if a[i] < b[i] {
                    flag = false;
                    break;
                }
            }
            if flag {
                res.push(word.clone());
            }
        }
        res
    }
}


fn main() {
    let words1 = vec!["amazon".to_string(), "apple".to_string(), "facebook".to_string(), "google".to_string()];
    let words2 = vec!["e".to_string(), "o".to_string()];
    let res = Solution::word_subsets(words1, words2);
    println!("{:?}", res);
}
