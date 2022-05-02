// https://leetcode.com/problems/shortest-word-distance-iii/submissions/
use std::cmp;
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
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

        if word1 == word2 {
            let mut shortest = i32::MAX;
            for i in 1..word1_idx_list.len() {
                shortest = cmp::min(shortest, word1_idx_list[i] - word1_idx_list[i - 1]);
            }

            return shortest;
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
                Ordering::Equal => panic!("wat"),
            };

            shortest = cmp::min(shortest, dist);
        }

        shortest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type S = Solution;

    #[test]
    fn test_basic() {
        assert_eq!(
            S::shortest_word_distance(
                vec!["hi", "ho", "hi"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect(),
                "hi".to_string(),
                "hi".to_string()
            ),
            2
        );

        assert_eq!(
            S::shortest_word_distance(
                vec!["hi", "ho", "mom", "hey", "ho", "mom"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect(),
                "hi".to_string(),
                "mom".to_string()
            ),
            2
        );
    }
}
