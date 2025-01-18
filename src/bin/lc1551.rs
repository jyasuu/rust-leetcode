struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
         let mut res = 0;
        for i in 0..n {
            res += (n - 2 * i - 1).abs();
        }
        res / 2
        
   }
}

fn main() {
    assert_eq!(0, Solution::min_operations(1));
    assert_eq!(1, Solution::min_operations(2));
    assert_eq!(2, Solution::min_operations(3));
    assert_eq!(6, Solution::min_operations(6));
}
