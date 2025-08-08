#![allow(unused)]
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n >= 4800 {
            return 1.0;
        }

        let mut memo = HashMap::new();
        Self::dfs((n + 24) / 25, (n + 24) / 25, &mut memo)
    }

    fn dfs(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
        if a <= 0 && b <= 0 {
            return 0.5;
        }
        if a <= 0 {
            return 1.0;
        }
        if b <= 0 {
            return 0.0;
        }
        if let Some(&res) = memo.get(&(a, b)) {
            return res;
        }

        let res = 0.25 * (
            Self::dfs(a - 4, b, memo) +
            Self::dfs(a - 3, b - 1, memo) +
            Self::dfs(a - 2, b - 2, memo) +
            Self::dfs(a - 1, b - 3, memo)
        );

        memo.insert((a, b), res);
        res
    }
}


fn main ()
{

}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn case1()
    {
        assert_eq!(Solution::soup_servings(50), 0.625);
    }
    
    #[test]
    fn case2()
    {
        assert_eq!(Solution::soup_servings(100), 0.71875);
    }
}