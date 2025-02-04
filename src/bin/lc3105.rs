struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut cur = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                cur += 1;
            } else {
                cur = 1;
            }
            res = res.max(cur);
        }
        let mut cur = 1;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                cur += 1;
            } else {
                cur = 1;
            }
            res = res.max(cur);
        }
        res
        
    }
}

fn main(){
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let res = Solution::longest_monotonic_subarray(nums);
    println!("{}", res);
}
