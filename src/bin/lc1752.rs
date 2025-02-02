struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut cnt = 0;
        for i in 0..n {
            if nums[i] > nums[(i+1)%n] {
                cnt += 1;
            }
        }
        cnt <= 1
        
    }
}

fn main(){
    let nums = vec![3,4,5,1,2];
    let res = Solution::check(nums);
    println!("{}", res);
}
