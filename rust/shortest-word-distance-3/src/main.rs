// https://leetcode.com/problems/shortest-word-distance-iii/submissions/
use std::cmp;
use std::cmp::Ordering;

struct Solution {}

impl Solution {
    fn build_list(words_dict: &Vec<String>, word: &str) -> Vec<i32> {
        let mut idx_list = Vec::new();
        for (i, w) in words_dict.iter().enumerate() {
            if *w == word {
                idx_list.push(i as i32);
            }
        }

        idx_list
    }

    fn single_word(words_dict: &Vec<String>, word: &str) -> i32 {
        let idx_list = Self::build_list(words_dict, word);
        let mut shortest = i32::MAX;
        for i in 1..idx_list.len() {
            let dist = idx_list[i] - idx_list[i - 1];
            shortest = cmp::min(dist, shortest);
        }

        shortest
    }

    fn double_word(words_dict: &Vec<String>, word1: &str, word2: &str) -> i32 {
        let idx_list_1 = Self::build_list(words_dict, word1);
        let idx_list_2 = Self::build_list(words_dict, word2);

        let mut i = 0;
        let mut j = 0;
        let mut shortest = i32::MAX;
        while i < idx_list_1.len() && j < idx_list_2.len() {
            let w1_idx = idx_list_1[i];
            let w2_idx = idx_list_2[j];
            let dist = match w1_idx.cmp(&w2_idx) {
                Ordering::Less => {
                    i += 1;
                    w2_idx - w1_idx
                }
                Ordering::Greater => {
                    j += 1;
                    w1_idx - w2_idx
                }
                Ordering::Equal => panic!("implies two words in one slot"),
            };

            shortest = cmp::min(dist, shortest);
        }

        shortest
    }

    pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        if word1 == word2 {
            Self::single_word(&words_dict, &word1)
        } else {
            Self::double_word(&words_dict, &word1, &word2)
        }
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
