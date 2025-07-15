#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut res = 0;
        if m % 2 == 1 {
            for i in 0..n {
                res ^= nums1[i];
            }
        }
        if n % 2 == 1 {
            for i in 0..m {
                res ^= nums2[i];
            }
        }

        res
    }
}

fn main() {}
