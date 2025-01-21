struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();

        let mut top_rem = grid[0].iter().map(|&num| num as i64).sum::<i64>();
        let mut bot_rem: i64 = 0;

        let mut min_score = i64::MAX;
        for i in 0..n {
            top_rem -= grid[0][i] as i64;
            if i > 0 {
                bot_rem += grid[1][i - 1] as i64;
            }
            min_score = min_score.min(top_rem.max(bot_rem));
            if bot_rem >= top_rem {break}
        }
        min_score
        
    }
}

fn main() {
    let grid = vec![vec![2, 5, 4], vec![1, 5, 1]];
    let res = Solution::grid_game(grid);
    println!("{}", res);
}
