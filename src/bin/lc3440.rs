#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut left = vec![0; n];
        let mut left_time = vec![0; n];
        let mut right = vec![0; n];
        let mut right_time = vec![0; n];

        let mut time = 0;
        let mut max = 0;
        for i in 0..n {
            left[i] = max;
            left_time[i] = start_time[i] - time;
            max = max.max(left_time[i]);
            time = end_time[i];
        }
        let mut max = 0;
        let mut time = event_time;
        for i in (0..n).rev() {
            right[i] = max;
            right_time[i] = time - end_time[i];
            max = max.max(right_time[i]);
            time = start_time[i];
        }
        let mut ans = 0;
        for i in 0..n {
            let duration = end_time[i] - start_time[i];
            if right[i] >= duration || left[i] >= duration {
                ans = ans.max(duration + left_time[i] + right_time[i]);
            } else {
                ans = ans.max(left_time[i] + right_time[i]);
            }
        }
        ans
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::max_free_time(21, vec![7, 10, 16], vec![10, 14, 18]),
            10
        );
    }
}
