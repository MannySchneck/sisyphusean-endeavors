use std::cmp;
use std::cmp::Ordering;

// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/submissions/

impl Solution {
    fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut word1_idx_list = Vec::new();
        let mut word2_idx_list = Vec::new();
        for (i, w) in words.iter().enumerate() {
            if *w == word1 {
                word1_idx_list.push(i as i32);
            }

            if *w == word2 {
                word2_idx_list.push(i as i32);
            }
        }

        let mut i = 0;
        let mut j = 0;
        let mut shortest = i32::MAX;
        while i < word1_idx_list.len() && j < word2_idx_list.len() {
            let i_val = word1_idx_list[i];
            let j_val = word2_idx_list[j];
            let dist = match i_val.cmp(&j_val) {
                Ordering::Less => {
                    i += 1;
                    j_val - i_val
                }
                Ordering::Greater => {
                    j += 1;
                    i_val - j_val
                }
                Ordering::Equal => panic!("Implies word collision"),
            };

            shortest = cmp::min(dist, shortest);
        }

        shortest
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    type S = Solution;

    #[test]
    fn test_next_to() {
        assert_eq!(
            S::shortest_distance(
                vec!["hi".to_string(), "mom".to_string()],
                "hi".to_string(),
                "mom".to_string()
            ),
            1
        );
    }

    #[test]
    fn test_distal_one() {
        assert_eq!(
            S::shortest_distance(
                vec!["hi".to_string(), "bob".to_string(), "mom".to_string()],
                "hi".to_string(),
                "mom".to_string()
            ),
            2
        );
    }

    #[test]
    fn test_multiple_possible() {
        assert_eq!(
            S::shortest_distance(
                vec![
                    "hi".to_string(),
                    "jack".to_string(),
                    "jim".to_string(),
                    "mom".to_string(),
                    "mom".to_string(),
                    "dad".to_string(),
                    "hi".to_string()
                ],
                "hi".to_string(),
                "mom".to_string()
            ),
            2
        );
    }
}
