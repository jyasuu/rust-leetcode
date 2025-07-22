#![allow(dead_code)]


struct Solution;
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        let mut l = 0;
        let mut r = 0;
        let n = nums.len();
        let mut ans = 0;
        let mut curr = 0;

        while r < n{
            while set.contains(&nums[r])
            {
                set.remove(&nums[l]);
                curr -= nums[l];
                l += 1;
                
            }
            set.insert(nums[r]);
            curr += nums[r];
            r += 1;
            ans = ans.max(curr);
        }
        
        ans
    }
}

fn main()
{

}


#[cfg(test)]
mod test
{
    use crate::*;

    #[test]
    fn case1()
    {
        assert_eq!(Solution::maximum_unique_subarray(vec![4,2,4,5,6]),17);
    }
    #[test]
    fn case2()
    {
        assert_eq!(Solution::maximum_unique_subarray(vec![5,2,1,2,5,2,1,2,5]),8);
    }
}