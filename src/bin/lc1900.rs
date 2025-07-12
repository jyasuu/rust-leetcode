#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        use std::i32;
        let players: Vec<i32> = (1..=n).collect();
        let mut min_round = i32::MAX;
        let mut max_round = 0;
        let mut hash = vec![vec![]; 29];
        fn compete_results(n: i32, hash: &mut Vec<Vec<Vec<bool>>>) -> Vec<Vec<bool>> {
            if n == 0 {
                return vec![];
            }
            if n == 1 {
                return vec![vec![true]];
            }
            if n == 2 {
                hash[n as usize] = vec![vec![true, false], vec![false, true]];
                return vec![vec![true, false], vec![false, true]];
            }
            let mut result = Vec::new();
            let compete_results = compete_results(n - 2, hash);

            for i in 0..compete_results.len() {
                let win_case = vec![true];
                let lose_case = vec![false];
                result.push([&win_case[..], &compete_results[i][..], &lose_case[..]].concat());
                result.push([&lose_case[..], &compete_results[i][..], &win_case[..]].concat());
            }
            hash[n as usize] = result.clone();
            result
        }
        compete_results(n, &mut hash);
        compete_results(n - 1, &mut hash);

        let mut competes = std::collections::VecDeque::new();
        competes.push_back(players);
        let mut round = 1;
        let mut prev_n = n;
        while let Some(compete) = competes.pop_front() {
            // println!("{:?}", compete);
            let current_n = compete.len();
            if current_n as i32 != prev_n {
                prev_n = current_n as i32;
                round += 1;
            }
            for i in 0..hash[current_n as usize].len() {
                let mut is_end = false;
                let mut is_valid = true;
                let mut next_compete = Vec::new();
                for j in 0..current_n {
                    if !hash[current_n as usize][i][j]
                        && (compete[j] == first_player || compete[j] == second_player)
                    {
                        is_valid = false;
                        break;
                    }

                    if compete[j] == first_player && compete[current_n - j - 1] == second_player {
                        max_round = max_round.max(round);
                        min_round = min_round.min(round);

                        is_end = true;
                        break;
                    }
                    if compete[current_n - j - 1] == first_player && compete[j] == second_player {
                        let round = 1 + ((n - current_n as i32) / 2);
                        max_round = max_round.max(round);
                        min_round = min_round.min(round);

                        is_end = true;
                        break;
                    }
                    if hash[current_n as usize][i][j] {
                        next_compete.push(compete[j]);
                    }
                }

                // println!("{:?} {} {}", hash[current_n as usize][i], is_end, is_valid);
                if is_valid && !is_end {
                    competes.push_back(next_compete);
                }
            }
        }

        vec![min_round, max_round]
    }
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn case1() {
        assert_eq!(Solution::earliest_and_latest(11, 2, 4), vec![3, 4]);
    }
}
