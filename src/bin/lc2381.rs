
struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut res = String::new();
        let mut shift_count = 0;
        let mut heap = std::collections::BinaryHeap::new();
        let mut shifts = shifts;
        shifts.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut j = 0;
        for i in 0..s.len()
        {
            while j < shifts.len() && shifts[j][0] <= i as i32
            {
                let shift = if shifts[j][2] == 1 {1} else { -1 };
                heap.push((std::cmp::Reverse(shifts[j][1]), shift));
                shift_count += shift;
                j += 1;
            }
            while let Some((std::cmp::Reverse(to_index), shift)) = heap.peek()
            {
                if *to_index < i as i32
                {
                    shift_count -= shift;
                    heap.pop();
                }
                else{
                    break;
                }
                
            }
            //println!("{j} {shift_count}");
            let mut c = s.chars().nth(i).unwrap() as i32 - 'a' as i32;
            c = (c + shift_count) % 26;
            c += 26;
            c = c % 26;
            res.push((c + 'a' as i32) as u8 as char);
            
        }
        
        res
    }
}

fn main()
{
    println!("{:?}", Solution::shifting_letters("abc".to_string(), vec![vec![0,1,1]]));
}