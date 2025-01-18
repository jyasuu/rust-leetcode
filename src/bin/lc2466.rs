struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut dp = vec![0; high as usize + 1];
        let mut res = 0;
        dp[zero as usize] += 1;
        dp[one as usize] += 1;
        const MOD: i32 = 1_000_000_007;
        // println!("{:?}", dp);
        for i in 0..=high as usize {
            if i >= zero as usize {
                dp[i] += dp[i - zero as usize];
            }

            if i >= one as usize {
                dp[i] += dp[i - one as usize];
            }
            dp[i] %= MOD;
            if low <= i as i32 {
                res += dp[i];
                res %= MOD;
            }
        }
        // println!("{:?}", dp);

        res
    }
}

fn main() {
    let low = 2;
    let high = 2;
    let zero = 1;
    let one = 1;
    let res = Solution::count_good_strings(low, high, zero, one);
    println!(r#"{}"#, res);
}
