struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let n = nums.len();

        for i in 1..n {
            if nums[i] % 2 == nums[i - 1] % 2 {
                return false;
            }
        }

        true
        
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let res = Solution::is_array_special(nums);
    println!("{}", res);
}
