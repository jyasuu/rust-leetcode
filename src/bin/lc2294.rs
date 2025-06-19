struct Solution;

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        while r < nums.len() {
            if nums[r] - nums[l] > k {
                l = r;
                ans += 1;
            }

            r += 1;
        }

        ans + 1
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::partition_array(vec![3, 6, 1, 2, 5], 2), 2);
    }
}
