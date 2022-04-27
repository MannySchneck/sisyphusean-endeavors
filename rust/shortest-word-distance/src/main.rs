use std::cmp;
use std::cmp::Ordering;

// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/submissions/

impl Solution {
    fn shortest_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut w1_idxs = Vec::new();
        let mut w2_idxs = Vec::new();
        for (i, w) in words.iter().enumerate() {
            if *w == word1 {
                w1_idxs.push(i as i32);
            }
            if *w == word2 {
                w2_idxs.push(i as i32);
            }
        }

        let mut shortest = i32::MAX;
        let mut i = 0;
        let mut j = 0;
        while i < w1_idxs.len() && j < w2_idxs.len() {
            let w1_idx = w1_idxs[i];
            let w2_idx = w2_idxs[j];
            let dist = match w1_idx.cmp(&w2_idx) {
                Ordering::Less => {
                    i += 1;
                    w2_idx - w1_idx
                },
                Ordering::Greater => {
                    j += 1;
                    w1_idx - w2_idx
                },
                Ordering::Equal => panic!("Implies two words in the same slot."),
            };

            shortest = cmp::min(shortest, dist);
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
        assert_eq!(S::shortest_distance(vec!["hi".to_string(), "mom".to_string()], "hi".to_string(), "mom".to_string()), 1);
    }

    #[test]
    fn test_distal_one() {
        assert_eq!(S::shortest_distance(vec!["hi".to_string(), "bob".to_string(), "mom".to_string()], "hi".to_string(), "mom".to_string()), 2);
    }

    #[test]
    fn test_multiple_possible() {
        assert_eq!(S::shortest_distance(
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
        ), 2);
    }
}
