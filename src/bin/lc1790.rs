struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let n = s1.len();
        let mut diff_count = 0;
        let mut prev_diff_index = -1;
        for i in 0..n
        {
            if s1.chars().nth(i) != s2.chars().nth(i)
            {
                diff_count += 1;
                if prev_diff_index == -1
                {
                    prev_diff_index = i as i32;
                }
                else
                {
                    if diff_count == 2
                    {
                        if s1.chars().nth(i) == s2.chars().nth(prev_diff_index as usize)
                        && s2.chars().nth(i) == s1.chars().nth(prev_diff_index as usize)
                        {

                        }
                        else{
                            return false;
                        }
                    }
                    else
                    {
                        return false;
                    }
                }
            }
        }

        diff_count == 0 || diff_count ==2
    }
}

fn main()
{
    Solution::are_almost_equal("bank".to_string(), "kanb".to_string());
}
