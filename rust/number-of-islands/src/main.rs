// https://leetcode.com/problems/number-of-islands/
struct Solution {}

impl Solution {
    fn erase_land(grid: &mut Vec<Vec<char>>, i: i64, j: i64) {
        if i < 0 || i == grid.len() as i64 || j < 0 || j == grid[0].len() as i64 {
            return;
        }

        if grid[i as usize][j as usize] == '0' {
            return;
        }

        grid[i as usize][j as usize] = '0';
        for k in [-1_i64, 1_i64] {
            if k == 0 {
                continue;
            }

            Self::erase_land(grid, i + k, j);
            Self::erase_land(grid, i, j + k);
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    Self::erase_land(&mut grid, i as i64, j as i64);
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type S = Solution;

    #[test]
    fn test_basic() {
        println!("hi");
        assert_eq!(S::num_islands(vec![vec!['0'], vec!['1']]), 1);
        assert_eq!(S::num_islands(vec![vec!['1'], vec!['1']]), 1);
        assert_eq!(S::num_islands(vec![vec!['1'], vec!['0'], vec!['1']]), 2);
        assert_eq!(
            S::num_islands(vec![vec!['1', '0'], vec!['1', '0'], vec!['0', '1']]),
            2
        );
    }
}
