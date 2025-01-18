struct Solution;


impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut count = 0;
        let mut is_counting = true;
        for c in s.chars() {
            if c == '*' && is_counting {
                count += 1;
            }
            if c == '|' {
                is_counting = !is_counting;
            }
        }
        count
        
    }
}
fn main()
{
    println!("{:?}", Solution::count_asterisks("*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*|*|*".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*|*|*|".to_string()));
    println!("{:?}", Solution::count_asterisks("*|*|*|*|*|*|*|*|*|*|*".to_string()));
 
}