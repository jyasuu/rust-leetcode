#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut players = players;
        let mut trainers = trainers;

        players.sort();
        trainers.sort();
        let n = players.len();
        let m = trainers.len();
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            while j < m && players[i] > trainers[j] {
                j += 1;
            }

            if i < n && j < m {
                // println!("{i} {j} ,{} {}", players[i], trainers[j]);
                if players[i] <= trainers[j] {
                    ans += 1;
                }
            }

            i += 1;
            j += 1;
        }

        ans
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::match_players_and_trainers(vec![4, 7, 9], vec![8, 2, 5, 8]),
            2
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            Solution::match_players_and_trainers(vec![1, 1, 1], vec![10]),
            1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            Solution::match_players_and_trainers(
                vec![2, 1],
                vec![2, 1, 2, 2, 3, 3, 2, 4, 1, 1, 4, 1, 3, 3, 4, 1, 3, 2, 3, 2, 2, 3, 1, 2, 4]
            ),
            2
        );
    }
}
