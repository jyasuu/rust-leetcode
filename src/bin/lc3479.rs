#![allow(dead_code)]
struct Solution;
struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(baskets: &Vec<i32>) -> Self {
        let size = baskets.len();
        let tree = vec![0; size * 4];
        let mut st = SegmentTree { size, tree };
        st.build(1, 0, size - 1, baskets);
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize, baskets: &Vec<i32>) {
        if l == r {
            self.tree[node] = baskets[l];
        } else {
            let mid = (l + r) / 2;
            self.build(node * 2, l, mid, baskets);
            self.build(node * 2 + 1, mid + 1, r, baskets);
            self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
        }
    }

    fn query(&self, node: usize, l: usize, r: usize, fruit: i32) -> Option<usize> {
        if self.tree[node] < fruit {
            return None;
        }
        if l == r {
            return Some(l);
        }
        let mid = (l + r) / 2;
        if let Some(left) = self.query(node * 2, l, mid, fruit) {
            return Some(left);
        }
        self.query(node * 2 + 1, mid + 1, r, fruit)
    }

    fn update(&mut self, node: usize, l: usize, r: usize, idx: usize) {
        if l == r {
            self.tree[node] = -1; // 標記為已使用
        } else {
            let mid = (l + r) / 2;
            if idx <= mid {
                self.update(node * 2, l, mid, idx);
            } else {
                self.update(node * 2 + 1, mid + 1, r, idx);
            }
            self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
        }
    }
}

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = baskets.len();
        let mut st = SegmentTree::new(&baskets);
        let mut unplaced = 0;

        for fruit in fruits {
            if let Some(idx) = st.query(1, 0, n - 1, fruit) {
                st.update(1, 0, n - 1, idx);
            } else {
                unplaced += 1;
            }
        }

        unplaced
    }
}

fn main()
{

}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn case1() {
        assert_eq!(Solution::num_of_unplaced_fruits(vec![4,2,5],vec![3,5,4]),1);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::num_of_unplaced_fruits(vec![3,6,1],vec![6,4,7]),0);

    }
}
