#![allow(dead_code)]

struct Solution;


impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut used = vec![false; baskets.len()];
        let mut unplaced = 0;

        for fruit in fruits {
            let mut placed = false;
            for (i, &basket) in baskets.iter().enumerate() {
                if !used[i] && basket >= fruit {
                    used[i] = true;
                    placed = true;
                    break;
                }
            }
            if !placed {
                unplaced += 1;
            }
        }

        unplaced
    }
}

fn main()
{

}


#[cfg(test)]
mod test{
    use crate::*;
    #[test]
    fn case1() {
        
        let fruits = vec![4, 2, 5];
        let baskets = vec![3, 5, 4];
        assert_eq!(Solution::num_of_unplaced_fruits(fruits, baskets), 1);
    }
}
