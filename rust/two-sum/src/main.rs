use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() < 2 {
        return vec![-1, -1];
    }

    let mut search_idx = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        search_idx.insert(n, i);
    }

    for (i, n) in nums.iter().enumerate() {
        let complement = target - n;
        match search_idx.get(&complement) {
            Some(j) => {
                if i != *j {
                    return vec![*j as i32, i as i32];
                }
            }
            None => continue,
        }
    }

    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_small() {
        assert_eq!(two_sum(vec![1, 1], 3), vec![-1, -1]);
    }

    #[test]
    fn test_obvious() {
        assert_eq!(two_sum(vec![1, 1], 2), vec![1, 0]);
    }

    #[test]
    fn test_less_obvious() {
        assert_eq!(two_sum(vec![1, 2, 3], 4), vec![2, 0]);
    }

    #[test]
    fn test_negatives() {
        assert_eq!(two_sum(vec![1, 5, -3, 4], 2), vec![2, 1]);
    }
}
