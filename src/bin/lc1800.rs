struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                sum = nums[i];
            }
            max_sum = max_sum.max(sum);
        }
        max_sum
    }
}

fn main(){
    assert_eq!(Solution::max_ascending_sum(vec![10,20,30,5,10,50]), 65);
    assert_eq!(Solution::max_ascending_sum(vec![10,20,30,40,50]), 150);
    assert_eq!(Solution::max_ascending_sum(vec![12,17,15,13,10,11,12]), 33);
    assert_eq!(Solution::max_ascending_sum(vec![100,10,1]), 100);
}
