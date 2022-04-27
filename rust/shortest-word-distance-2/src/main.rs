use std::collections::HashMap;
use std::cmp;
use std::cmp::Ordering;

impl WordDistance {

    fn new(wordsDict: Vec<String>) -> Self {
        let mut wd = WordDistance{
            word_idxs: HashMap::new(),
        };

        for (i, w) in wordsDict.into_iter().enumerate() {
            wd.word_idxs.entry(w)
                .and_modify(| e | e.push(i as i32))
                .or_insert(vec![i as i32]);
        }

        wd
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        let word1_idxs = match self.word_idxs.get(&word1) {
            Some(idxs) => idxs,
            None => return -1,
        };
        let word2_idxs = match self.word_idxs.get(&word2) {
            Some(idxs) => idxs,
            None => return -1,
        };

        let mut i = 0;
        let mut j = 0;
        let mut shortest = i32::MAX;

        while i < word1_idxs.len() && j < word2_idxs.len() {
            let i_val = word1_idxs[i];
            let j_val = word2_idxs[j];

            let dist = match i_val.cmp(&j_val) {
                Ordering::Less => {
                    i += 1;
                    j_val - i_val
                },
                Ordering::Greater => {
                    j += 1;
                    i_val - j_val
                },
                Ordering::Equal => panic!("Implies two words in one vec slot"),
            };

            shortest = cmp::min(shortest, dist);
        }

        shortest
   }
}

#[derive(Debug)]
struct WordDistance {
    word_idxs: HashMap<String, Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    type WD = WordDistance;

    #[test]
    fn test_basic() {
        let wd = WD::new(vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]);
        assert_eq!(wd.shortest("foo".to_string(), "bar".to_string()), 1);
        assert_eq!(wd.shortest("foo".to_string(), "baz".to_string()), 2);
    }
}
