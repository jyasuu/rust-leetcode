struct Solution;

impl Solution {
    pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
        let mut res = Vec::new();
        let chars = s.as_bytes();
        
        for i in 0..s.len()
        {
            let mut count = 0;
            let mut pos = start_pos.clone();
            for j in i..s.len()
            {
                match chars[j] {
                    b'U' => {
                        pos[0] -= 1;
                    }
                    b'D' => {
                        pos[0] += 1;
                    }
                    b'L' => {
                        pos[1] -= 1;
                    }
                    b'R' => {
                        pos[1] += 1;
    
                    }
                    _ => {}
                }
                if pos[0] >= 0  && pos[0] < n && pos[1] >= 0 && pos[1] < n
                {
                    count += 1;    
                }
                else{
                    break;
                }
                
            }
            res.push(count);
            
        }
        res
    }
}

fn main()
{
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RRU".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "URUR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUUR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRRRRRRR".to_string()));
    println!("{:?}", Solution::execute_instructions(5, vec![0,0], "RUURRRRRRRRRRRRRR".to_string()));
    
}