struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut map = vec![vec![0,0];arr.len()];
        for i in 0..m
        {
            for j in 0..n
            {
                map[mat[i][j] as usize -1][0] = i;
                map[mat[i][j] as usize -1][1] = j;
            }
        }
        //println!("map {:?}",map);
        let mut row_count = vec![0;m];
        let mut col_count = vec![0;n];
        for i in 0..arr.len()
        {
            row_count[map[arr[i] as usize -1][0]] += 1;
            if row_count[map[arr[i] as usize -1][0]] == n
            {
                return i as i32;
            }

            col_count[map[arr[i] as usize -1][1]] += 1;
            if col_count[map[arr[i] as usize -1][1]] == m
            {
                return i as i32;
            }
            //println!("row_count {:?}",row_count);
            //println!("col_count {:?}",col_count);

        }

        0

    }
}

fn main()
{

}
