struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for i in 1..points.len() {
            let x = (points[i][0] - points[i - 1][0]).abs();
            let y = (points[i][1] - points[i - 1][1]).abs();
            res += x.max(y);
        }
        res
        
    }
}

fn main() {
    let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
    let res = Solution::min_time_to_visit_all_points(points);
    println!("{}", res);
}
