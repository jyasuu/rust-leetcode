#![allow(dead_code)]
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn trap_rain_water(h: Vec<Vec<i32>>) -> i32 {
        let n = h.len();
        let m = h[0].len();
        let mut level = vec![vec![i32::MAX; m]; n];
        let mut q = BinaryHeap::new();
        for i in 0..n {
            for &j in &[0, m - 1] {
                level[i][j] = h[i][j];
                q.push((Reverse(level[i][j]), i, j))
            }
        }
        for j in 1..m - 1 {
            for &i in &[0, n - 1] {
                level[i][j] = h[i][j];
                q.push((Reverse(level[i][j]), i, j))
            }
        }
        while let Some((Reverse(x), i, j)) = q.pop() {
            let ip = i + 1;
            let im = i.wrapping_sub(1);
            let jp = j + 1;
            let jm = j.wrapping_sub(1);
            for &(ii, jj) in &[(ip, j), (im, j), (i, jp), (i, jm)] {
                if ii < n && jj < m && level[ii][jj] == i32::MAX {
                    level[ii][jj] = x;
                    q.push((Reverse(x.max(h[ii][jj])), ii, jj))
                }
            }
        }
        let mut total = 0;
        for i in 0..n {
            for j in 0..m {
                total += (level[i][j] - h[i][j]).max(0)
            }
        }
        total
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn case1() {
        assert_eq!(
            Solution::trap_rain_water(vec![
                vec![1, 4, 3, 1, 3, 2],
                vec![3, 2, 1, 3, 2, 4],
                vec![2, 3, 3, 2, 3, 1]
            ]),
            4
        );
    }
}
