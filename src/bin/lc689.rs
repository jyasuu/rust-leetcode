
struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        // Prefix sum array to calculate sum of any subarray in constant time
        let mut prefix_sum = vec![0;n + 1];
        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1];
        }

        // Arrays to store the best sum and starting indices for up to 3 subarrays
        let mut best_sum = vec![vec![0;n + 1];4];
        let mut best_index = vec![vec![0;n + 1];4];

        // Compute the best sum and indices for 1, 2, and 3 subarrays
        for subarray_count in 1..=3 
        {
            for end_index in k * subarray_count..=n
            {
                let current_sum =
                    prefix_sum[end_index] -
                    prefix_sum[end_index - k] +
                    best_sum[subarray_count - 1][end_index - k];

                // Check if the current configuration gives a better sum
                if current_sum > best_sum[subarray_count][end_index - 1] {
                    best_sum[subarray_count][end_index] = current_sum;
                    best_index[subarray_count][end_index] = end_index - k;
                } else {
                    best_sum[subarray_count][end_index] =
                        best_sum[subarray_count][end_index - 1];
                    best_index[subarray_count][end_index] =
                        best_index[subarray_count][end_index - 1];
                }
            }
        }

        // Trace back the indices of the three subarrays
        let mut result = vec![0;3];
        let mut current_end = n;
        for subarray_index in (1..=3).rev() {
            result[subarray_index - 1] = best_index[subarray_index][current_end] as i32;
            current_end = result[subarray_index - 1] as usize;
        }

        result
    }
}




// class Solution {

//     public int[] maxSumOfThreeSubarrays(int[] nums, int k) {
//         int n = nums.length;

//         // Prefix sum array to calculate sum of any subarray in constant time
//         int[] prefixSum = new int[n + 1];
//         for (int i = 1; i <= n; i++) {
//             prefixSum[i] = prefixSum[i - 1] + nums[i - 1];
//         }

//         // Arrays to store the best sum and starting indices for up to 3 subarrays
//         int[][] bestSum = new int[4][n + 1];
//         int[][] bestIndex = new int[4][n + 1];

//         // Compute the best sum and indices for 1, 2, and 3 subarrays
//         for (int subarrayCount = 1; subarrayCount <= 3; subarrayCount++) {
//             for (int endIndex = k * subarrayCount; endIndex <= n; endIndex++) {
//                 int currentSum =
//                     prefixSum[endIndex] -
//                     prefixSum[endIndex - k] +
//                     bestSum[subarrayCount - 1][endIndex - k];

//                 // Check if the current configuration gives a better sum
//                 if (currentSum > bestSum[subarrayCount][endIndex - 1]) {
//                     bestSum[subarrayCount][endIndex] = currentSum;
//                     bestIndex[subarrayCount][endIndex] = endIndex - k;
//                 } else {
//                     bestSum[subarrayCount][endIndex] =
//                         bestSum[subarrayCount][endIndex - 1];
//                     bestIndex[subarrayCount][endIndex] =
//                         bestIndex[subarrayCount][endIndex - 1];
//                 }
//             }
//         }

//         // Trace back the indices of the three subarrays
//         int[] result = new int[3];
//         int currentEnd = n;
//         for (int subarrayIndex = 3; subarrayIndex >= 1; subarrayIndex--) {
//             result[subarrayIndex - 1] = bestIndex[subarrayIndex][currentEnd];
//             currentEnd = result[subarrayIndex - 1];
//         }

//         return result;
//     }
// }
fn main() {
    let nums = vec![1,2,1,2,6,7,5,1];
    let k = 2;
    let res = Solution::max_sum_of_three_subarrays(nums, k);
    println!("{:?}", res);
}