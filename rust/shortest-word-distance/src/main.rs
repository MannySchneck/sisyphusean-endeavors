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

        let mut i = 0;
        let mut j = 0;
        let mut smallest = i32::MAX;
        while i < w1_idxs.len() && j < w2_idxs.len() {
            let i_val = w1_idxs[i];
            let j_val = w2_idxs[j];
            let dist = match i_val.cmp(&j_val) {
                Ordering::Less => {
                    i += 1;
                    j_val - i_val
                },
                Ordering::Greater => {
                    j += 1;
                    i_val - j_val
                },
                Ordering::Equal => panic!("implies two words in one slot"),
            };

            smallest = cmp::min(dist, smallest);
        }

        smallest
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
