struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut vec = vec![0; n + 1];
        let mut res = Vec::new();
        let mut count = 0;
        for i in 0..n {
            let a_val = a[i];
            let b_val = b[i];
            if vec[a_val as usize] > 0
            {
                count += 1;
            }
            vec[a_val as usize] += 1;
            if vec[b_val as usize] > 0
            {
                count += 1;
            }
            vec[b_val as usize] += 1;
 
            res.push(count);
        }


        
        res
    }
}

fn main()
{
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![2, 1, 3, 4, 5];
    let res = Solution::find_the_prefix_common_array(a, b);
    for i in res.iter()
    {
        print!("{} ", i);
    }
    println!();

}
