#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3{
            return false;
        }
        let mut digits = 0;
        let mut vowel = 0;
        let mut consonant = 0;
        for byte in word.bytes()
        {
            match byte
            {
                48..=57 => {digits += 1;},
                97..=122 => {
                    if byte == 97 || byte == 101 || byte == 105 || byte == 111 || byte == 117
                    {
                        vowel += 1;

                    }
                    else{
                        consonant += 1;
                    }
                },
                65..=90 => {
                    if byte == 97 - 32 || byte == 101 -32 || byte == 105 -32 || byte == 111 -32 || byte == 117 -32
                    {
                        vowel += 1;

                    }
                    else{
                        consonant += 1;
                    }
                },
                _ => {return false;}

            }
        }
        if vowel == 0 || consonant == 0
        {
            return false;
        }


       true
    }
}
fn main()
{
    println!("{}",b'0');
    println!("{}",b'9');
    println!("{} {}",b'a',b'z');
    println!("{} {}",b'A',b'Z')

}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn case1()
    {
        assert_eq!(Solution::is_valid("234Adas".to_string()), true);
    }

}

