struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut result_state = vec![0; n];
        
        fn is_safe(node: usize, graph: &Vec<Vec<i32>>, state: &mut Vec<i32>) -> bool {
            if state[node] != 0 {
                return state[node] == 2;
            }
            state[node] = 1; // Mark the node as visiting
            for &neighbor in &graph[node] {
                if state[neighbor as usize] == 1 || !is_safe(neighbor as usize, graph, state) {
                    return false;
                }
            }
            state[node] = 2; // Mark the node as safe
            true
        }

        let mut safe_nodes = vec![];
        for i in 0..n {
            if is_safe(i, &graph, &mut result_state) {
                safe_nodes.push(i as i32);
            }
        }

        safe_nodes.sort_unstable();
        safe_nodes       
    }
}

fn main()
{
    let graph = vec![vec![1,2], vec![2,3], vec![5], vec![0], vec![], vec![]];
    let res = Solution::eventual_safe_nodes(graph);
    println!("{:?}", res);
}
