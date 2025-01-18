struct Solution;


impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
         derived.iter().fold(0,|n,bit| n ^ bit) == 0

    }
}

fn main()
{
    println!("{:#?}",Solution::does_valid_array_exist(vec![1,1,0]));
    println!("{:#?}",Solution::does_valid_array_exist(vec![1,1]));
    println!("{:#?}",Solution::does_valid_array_exist(vec![1,0]));
}
