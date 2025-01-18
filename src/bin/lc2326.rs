
// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 
 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

struct Solution;

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![-1; n as usize]; m as usize];
        let mut head = head;
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;
        let mut r = n - 1;
        let mut l = 0;
        let mut d = m - 1;
        let mut u = 1;
        while let Some(mut node) = head {
            res[x as usize][y as usize] = node.val;
            match dir {
                0 => {
                    if y == r {
                        dir = 1;
                        r -= 1;
                        x += 1;
                    } else {
                        y += 1;
                    }
                }
                1 => {
                    if x == d {
                        dir = 2;
                        d -= 1;
                        y -= 1;
                    } else {
                        x += 1;
                    }
                }
                2 => {
                    if y == l {
                        dir = 3;
                        l += 1;
                        x -= 1;
                    } else {
                        y -= 1;
                    }
                }
                3 => {
                    if x == u {
                        dir = 0;
                        u += 1;
                        y += 1;
                    } else {
                        x -= 1;
                    }
                }
                _ => {}
            }
            head = node.next;
        }


        res        
    }
}

fn main()
{

}
