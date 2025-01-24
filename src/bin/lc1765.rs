struct Solution;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![-1; is_water[0].len()]; is_water.len()];
        let mut q = Vec::new();
        for i in 0..is_water.len() {
            for j in 0..is_water[i].len() {
                if is_water[i][j] == 1 {
                    res[i][j] = 0;
                    q.push((i, j));
                }
            }
        }
        let dirs = [-1, 0, 1, 0, -1];
        while !q.is_empty() {
            let mut new_q = Vec::new();
            for (i, j) in q {
                for k in 0..4 {
                    let x = i as i32 + dirs[k];
                    let y = j as i32 + dirs[k + 1];
                    if x >= 0 && x < is_water.len() as i32 && y >= 0 && y < is_water[0].len() as i32 && res[x as usize][y as usize] == -1 {
                        res[x as usize][y as usize] = res[i][j] + 1;
                        new_q.push((x as usize, y as usize));
                    }
                }
            }
            q = new_q;
        }
        res
        
    }
}

fn main()
{
    let is_water = vec![vec![0, 1], vec![0, 0]];
    let res = Solution::highest_peak(is_water);
    for i in 0..res.len() {
        for j in 0..res[i].len() {
            print!("{} ", res[i][j]);
        }
        println!();
    }
}
