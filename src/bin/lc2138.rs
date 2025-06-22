struct Solution;
impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let mut ans = Vec::new();
        let mut i = 0;
        let mut temp = String::new();
        for c in s.chars() {
            i += 1;
            temp.push(c);

            if i == k {
                ans.push(temp);
                i = 0;
                temp = String::new();
            }
        }
        while i > 0 && i < k {
            temp.push(fill);
            i += 1;
            if i == k {
                ans.push(temp);
                temp = String::new();
            }
        }
        ans
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn test() {
        let s = "abcdefghi";
        let k = 3;
        let fill = 'x';
        assert_eq!(
            Solution::divide_string(s.to_string(), k, fill),
            vec!["abc", "def", "ghi"]
        );
    }
    #[test]
    fn test2() {
        let s = "abcdefghij";
        let k = 3;
        let fill = 'x';
        assert_eq!(
            Solution::divide_string(s.to_string(), k, fill),
            vec!["abc", "def", "ghi", "jxx"]
        );
    }
}
