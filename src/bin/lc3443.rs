#[warn(dead_code)]
struct Solution;

impl Solution {
    pub fn max_distance(str: String, k: i32) -> i32 {
        let mut n: i32 = 0;
        let mut s: i32 = 0;
        let mut e: i32 = 0;
        let mut w: i32 = 0;
        let chars = str.chars();

        let mut max = 0;
        for c in chars {
            match c {
                'N' => {
                    n += 1;
                }
                'S' => {
                    s += 1;
                }
                'E' => {
                    e += 1;
                }
                'W' => {
                    w += 1;
                }
                _ => {}
            }

            max = max.max(n + e - w - s + 2 * std::cmp::min(s + w, k));
            max = max.max(n + w - s - e + 2 * std::cmp::min(s + e, k));
            max = max.max(s + e - n - w + 2 * std::cmp::min(n + w, k));
            max = max.max(s + w - n - e + 2 * std::cmp::min(n + e, k));
        }
        max
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    #[test]
    fn name() {
        assert_eq!(Solution::max_distance(String::from("NWSE"), 1), 3);
    }
}

fn main() {}
