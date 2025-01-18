struct Solution;


impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![-1; n];
        let mut stack = vec![];
        for i in 0..2*n {
            let num = nums[i % n];
            while !stack.is_empty() && nums[*stack.last().unwrap()] < num {
                res[stack.pop().unwrap()] = num;
            }
            if i < n {
                stack.push(i);
            }
        }
        res
    }
}


fn main() {
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 1]), vec![2, -1, 2]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 3]), vec![2, 3, 4, -1, 4]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 5]), vec![2, 3, 4, 5, -1]);
    assert_eq!(Solution::next_greater_elements(vec![5, 4, 3, 2, 1]), vec![-1, 5, 5, 5, 5]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), vec![2, 3, 4, 5, 6, 7, 8, 9, 10, -1]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1]), vec![2, 3, 4, 5, 6, 7, 8, 9, 10, -1, 2]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10]), vec![2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -1]);
    assert_eq!(Solution::next_greater_elements(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9]), vec![2, 3, 4, 5, 6, 7, 8, 9, 10, -1, 10]);
}
