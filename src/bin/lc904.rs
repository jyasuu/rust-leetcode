#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut map = std::collections::HashMap::new();
        let mut ans = 0;

        for r in 0..fruits.len() {
            *map.entry(fruits[r]).or_insert(0) += 1;

            while map.len() > 2 {
                map.get_mut(&fruits[l]).map(|n| *n -= 1);
                let n = map.get(&fruits[l]).unwrap();
                if *n == 0 {
                    map.remove(&fruits[l]);
                }
                l += 1;
            }
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }
    #[test]
    fn case3() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }
}
