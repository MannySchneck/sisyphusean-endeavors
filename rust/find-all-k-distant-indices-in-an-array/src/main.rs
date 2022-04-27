use std::cmp;

// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/submissions/

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut key_indices = Vec::new();

        for (i, n) in nums.iter().enumerate() {
            if *n == key {
                key_indices.push(i);
            }
        }

        let mut res = Vec::new();
        for n in key_indices.iter() {
            left = {
                let computed = n - k;
                let unchecked = match res.last() {
                    Some(n) => cmp::max(n + 1, computed),
                    None => computed,
                };

                cmp::max(0, unchecked)
            };

            let right = cmp::min(nums.len() as i32 - 1, n + k);

            for i in left..=right {
                res.push(i);
            }
        }

        res
    }
}
