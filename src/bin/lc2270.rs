struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i] as i64;
        }
        let mut count = 0;
        let mut res = 0;
        for i in 0..nums.len()-1 {
            count += nums[i] as i64;
            if count >= sum - count {
                res += 1;
            }
        }        
        res
    }
}

fn main()
{
    let nums = vec![1, 2, 2, 2, 5, 0];
    let res = Solution::ways_to_split_array(nums);
    println!("{}", res);
    
}