// https://leetcode.com/problems/shortest-word-distance-iii/submissions/
use std::cmp;
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn build_idx(words_dict: &Vec<String>, word: &str) -> Vec<i32> {
        let mut idx = Vec::new();
        for (i, w) in words_dict.iter().enumerate() {
            if *w == word {
                idx.push(i as i32);
            }
        }

        idx
    }

    pub fn shortest_word_distance(
        words_dict: Vec<String>,
        word1: String,
        word2: String
    ) -> i32 {
        if word1 == word2 {
            let word_idx = Self::build_idx(&words_dict, &word1);

            if word_idx.len() < 2 {
                panic!("precondition violation");
            }

            let mut shortest = i32::MAX;
            for i in 1..word_idx.len() {
                let dist = word_idx[i] - word_idx[i - 1];
                shortest = cmp::min(dist, shortest);
            }

            return shortest
        }

        let word1_idx = Self::build_idx(&words_dict, &word1);
        let word2_idx = Self::build_idx(&words_dict, &word2);

        let mut i = 0;
        let mut j = 0;
        let mut shortest = i32::MAX;
        while i < word1_idx.len() && j < word2_idx.len() {
            let i_val = word1_idx[i];
            let j_val = word2_idx[j];

            let dist = match i_val.cmp(&j_val) {
                Ordering::Less => {
                    i += 1;
                    j_val - i_val
                },
                Ordering::Greater => {
                    j += 1;
                    i_val - j_val
                },
                Ordering::Equal => {
                    panic!("two words in the same slot");
                }
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
                vec!["hi", "ho", "hi"].iter().map(|e| e.to_string()).collect(),
                "hi".to_string(),
                "hi".to_string()
            ),
            2
        );
    }
}
