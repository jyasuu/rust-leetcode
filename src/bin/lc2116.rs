#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n = s.len();
        if n % 2 != 0 {
            return false;
        }

        let bytes = s.as_bytes();
        let locked = locked.as_bytes();

        let mut l_count = 0;
        let mut r_count = 0;
        let mut x_count = 0;
        for i in 0..n {
            match locked[i] {
                b'0' => {
                    x_count += 1;
                }
                b'1' => match bytes[i] {
                    b'(' => {
                        l_count += 1;
                    }
                    b')' => {
                        r_count += 1;
                    }
                    _ => {}
                },
                _ => {}
            }
            if r_count > l_count + x_count {
                return false;
            }
        }

        let mut l_count = 0;
        let mut r_count = 0;
        let mut x_count = 0;
        for i in (0..n).rev() {
            match locked[i] {
                b'0' => {
                    x_count += 1;
                }
                b'1' => match bytes[i] {
                    b'(' => {
                        l_count += 1;
                    }
                    b')' => {
                        r_count += 1;
                    }
                    _ => {}
                },
                _ => {}
            }
            if l_count > r_count + x_count {
                return false;
            }
        }

        true
    }
}

fn main() {}
