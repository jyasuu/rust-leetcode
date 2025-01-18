struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse((0,0,0)));
        let mut cost_grid = vec![vec![i32::MAX;m];n];

        let directions = [(0,0),(0,1),(0,-1),(1,0),(-1,0)];

        while let Some(std::cmp::Reverse((cost,x,y))) = heap.pop()
        {
            if cost >= cost_grid[x][y]
            {
                continue;
            }
            cost_grid[x][y] = cost;

            if n - 1 == x && m - 1 == y
            {
                break;
            }
            
            for (i , direction) in directions.iter().enumerate().skip(1)
            {
                let next_cost = if grid[x][y] == i as i32 { cost } else { cost + 1};
                let next_x = x as i32 + direction.0;
                let next_y = y as i32 + direction.1;
                if next_x >= 0 && next_x < n as i32 && next_y >= 0 && next_y < m as i32
                {
                    heap.push(std::cmp::Reverse((next_cost , next_x as usize , next_y as usize)));
                }


            }
        }



        cost_grid[n - 1][m - 1]
    }
}

fn main()
{

}
