struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days.len();
        let mut next = vec![0; 31];
        let mut curr = vec![0; 31];

        for ind in (0..n).rev() {
            for left_days in (0..=30).rev() {
                if left_days > 0 && ind > 0 && (days[ind] - days[ind - 1]) <= left_days as i32 {
                    curr[left_days] = next[left_days - (days[ind] - days[ind - 1]) as usize];
                } else {
                    let one = costs[0] + next[0];
                    let seven = costs[1] + next[6];
                    let thirty = costs[2] + next[29];

                    curr[left_days] = one.min(seven).min(thirty);
                }
            }
            next.copy_from_slice(&curr);
        }
        next[0]
        // let mut dp = vec![i32::MAX; 366];
        // let mut dp2 = vec![false; 366];
        // for &day in days.iter(){
        //     dp2[day as usize] = true;
        // }
        // let mut heap = std::collections::BinaryHeap::new();
        // heap.push((std::cmp::Reverse(1),0));
        // while let Some((std::cmp::Reverse(day), cost)) = heap.pop(){
        //     let mut i = day as usize - 1;
        //     while i > 0
        //     {
        //         if i > 365 {
        //             i -= 1;
        //             continue;
        //         }
        //         if dp[i] > cost{
        //             dp[i] = cost;
        //         }
        //         else{
        //             break;
        //         }
        //         i -= 1;
        //     }
        //     if day > 365 {
        //         break;
        //     }
        //     if dp2[day as usize]{
        //         if dp[day as usize] < cost{
        //             continue;
        //         }
        //         dp[day as usize] = cost + 2;
        //         heap.push((std::cmp::Reverse(day + 1), cost + costs[0]));
        //         heap.push((std::cmp::Reverse(day + 7), cost + costs[1]));
        //         heap.push((std::cmp::Reverse(day + 30), cost + costs[2]));
        //     }else{
        //         if dp[day as usize] < cost{
        //             continue;
        //         }
        //         dp[day as usize] = cost;
        //         heap.push((std::cmp::Reverse(day + 1), cost));
        //     }
        // }

        // dp[*days.last().unwrap() as usize]
        
    }
}

fn main(){
    let days = vec![1,4,6,7,8,20];
    let costs = vec![2,7,15];
    let res = Solution::mincost_tickets(days, costs);
    println!("{}", res);
    let days = vec![1,4,6,7,8,365];
    let costs = vec![2,7,15];
    let res = Solution::mincost_tickets(days, costs);
    println!("{}", res);
}