struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut n = num2.count_ones();
        let mut vec = Vec::new();
        let mut m = num1;
        
        while m > 0
        {
            vec.push(m % 2 == 1);
            m /= 2;
        }
        let mut bits = vec![false;vec.len()];
        for i in (0..vec.len()).rev()
        {
            if n == 0
            {
                break;
            }

            if vec[i]
            {
                vec[i] = false;
                bits[i] = true;
                n -= 1;
            }

        }
        let mut i = 0;
        
        
        while n > 0
        {
            if i == bits.len()
            {
                bits.push(true);
                n -= 1;

            }
            else 
            {
                if !bits[i]
                {
                    bits[i] = true;
                    n -= 1;
                }            
            }
            
            i += 1;
        }
        let mut res = 0;
        // println!("{:?}",bits);
        for i in 0..bits.len()
        {
            if bits[i]
            {
                res += 1 << i;
            }
        }

        res
    }
}

fn main()
{

}


// 25
// 11001
// 72
// 1001000
