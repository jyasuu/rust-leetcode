#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let even_count = nums.iter().filter(|&&x| x % 2 == 0).count();
        let odd_count = nums.iter().filter(|&&x| x % 2 == 1).count();
        let mut ans = even_count.max(odd_count) as i32;

        let mut count = 0;
        let mut prev = 0;
        for i in 0..n
        {
            if nums[i] % 2 == 1 && prev % 2 == 0
            {
                count += 1;
                prev = nums[i];
            }
            
            if nums[i] % 2 == 0 && prev % 2 == 1
            {
                count += 1;
                prev = nums[i];
            }

        }
        ans = ans.max(count) as i32;
        
        let mut count = 0;
        let mut prev = 1;
        for i in 0..n
        {
            if nums[i] % 2 == 1 && prev % 2 == 0
            {
                count += 1;
                prev = nums[i];
            }
            
            if nums[i] % 2 == 0 && prev % 2 == 1
            {
                count += 1;
                prev = nums[i];
            }

        }
        ans = ans.max(count) as i32;
        // let mut dp2 = vec![0;n];

        //let mut dp = vec![vec![0;n];n];
        // for i in 1..n
        // {
        //     for j in 0..i
        //     {
        //         if (nums[i] + nums[j]) % 2 == 1
        //         {

        //             //dp[i][j] = 1;
        //             if j > 0 {
        //                 //dp[i][j] += dp[j][j - 1];
        //                 dp2[i] = dp2[i].max( dp2[j] + 1);
        //             }
        //             else{
        //                 dp2[i] = 1;
        //             }
                    
        //         }
        //         else{
        //             //dp[i][j] = dp[i - 1][j];
        //             //if j > 0 {
        //             //    dp[i][j] = dp[i][j].max(dp[i][j - 1]);
        //             //}
        //         }
        //     }
        // }

        // //ans = ans.max(dp[n - 1][n - 2] + 1) as i32;
        // ans = ans.max(dp2[n - 1] + 1) as i32;

        ans
    }
}

fn main(){}

#[cfg(test)]
mod test
{
    use crate::*;

    #[test]
    fn case1()
    {
        assert_eq!(Solution::maximum_length((1..=10000).collect()) ,10000);
    }

    #[test]
    fn case2()
    {
        assert_eq!(Solution::maximum_length(vec![1,2,1,1,2,1,2]) ,6);
    }

    #[test]
    fn case3()
    {
        assert_eq!(Solution::maximum_length(vec![1,7,6,2,9]) ,3);
    }
}