struct Solution;

impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut single = 0;
        let mut double = 0;
        for num in nums {
            if num < 10 {
                single += num;
            } 
            else {
                double += num;
            }
        }
        single != double
    }
}

fn main(){
    let nums = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
    let result = Solution::can_alice_win(nums);
    println!("{}", result);
}

