use std::cmp;

// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/submissions/

struct Solution {}

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut key_indices = Vec::new();
        for (i, n) in nums.iter().enumerate() {
            if *n == key {
                key_indices.push(i as i32);
            }
        }

        let mut res = Vec::new();
        for key_index in key_indices {
            let left = {
                let computed = cmp::max(key_index - k, 0);
                match res.last() {
                    Some(n) => cmp::max(n + 1, computed),
                    None => computed,
                }
            };

            let right = cmp::min(nums.len() as i32 - 1 as i32, key_index + k);

            for i in left..=right {
                res.push(i);
            }
        }

        res
    }
}
