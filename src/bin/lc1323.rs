struct Solution;

type Maximum69NumberFn = fn(num: i32) -> i32;

impl Solution {
    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn maximum69_number (num: i32) -> i32 {
        for i in 0..num.to_string().len() {
            if num.to_string().chars().nth(i).unwrap() == '6' {
                return num + 3 * 10_i32.pow((num.to_string().len() - i - 1) as u32);
            }
        }
        num
    }
}

fn main ()
{
    let num = 9669;
    let result = Solution::maximum69_number(num);
    println!("{:?}", result);
    let ptr: Maximum69NumberFn = Solution::maximum69_number;
    println!("{:?}", ptr(num));
}