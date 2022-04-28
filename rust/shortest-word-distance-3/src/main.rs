// https://leetcode.com/problems/shortest-word-distance-iii/submissions/
use std::cmp;
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn build_idx_list(words_dict: &Vec<String>, word: &str) -> Vec<i32> {
        let mut idx_list = Vec::new();
        for (i, w) in words_dict.iter().enumerate() {
            if *w == word {
                idx_list.push(i as i32);
            }
        }

        idx_list
    }

    fn single_word(words_dict: &Vec<String>, word: &str) -> i32 {
        let idx_list = Self::build_idx_list(words_dict, word);
        let mut shortest = i32::MAX;
        for i in 1..idx_list.len() {
            let dist = idx_list[i] - idx_list[i - 1];
            shortest = cmp::min(dist, shortest);
        }

        shortest
    }

    pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        if word1 == word2 {
            return Self::single_word(&words_dict, &word2);
        }

        let w1_idx_list = Self::build_idx_list(&words_dict, &word1);
        let w2_idx_list = Self::build_idx_list(&words_dict, &word2);

        let mut i = 0;
        let mut j = 0;
        let mut shortest = i32::MAX;
        while i < w1_idx_list.len() && j < w2_idx_list.len() {
            let i_val = w1_idx_list[i];
            let j_val = w2_idx_list[j];
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

            shortest = cmp::min(dist, shortest);
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
