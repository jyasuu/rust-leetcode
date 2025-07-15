#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![0; grid.len()];
        let mut col = vec![0; grid[0].len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }
        let mut result = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && (row[i] > 1 || col[j] > 1) {
                    result += 1;
                }
            }
        }
        result
    }
}

fn main() {
    let grid = vec![vec![1, 0], vec![0, 1]];
    let result = Solution::count_servers(grid);
    println!("{}", result);
}
