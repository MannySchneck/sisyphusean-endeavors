// https://leetcode.com/problems/shortest-word-distance-iii/submissions/

struct Solution {}

impl Solution {
    /* The tokens on the board satisfy: |x| = |o| OR |x| = | o + 1 |
     *
     * if x wins, count(x) = count(o) + 1
     * if o wins, count(o) = count(x) -- must check that x didn't win.
     * can't have two disjoint (top/bottom) win states.
     */

    fn did_win(player: u8, board: &Vec<String>) -> bool {
        let mut won_down = true;
        let mut won_up = true;
        for i in 0..3 {
            // Diagonals
            if board[i].as_bytes()[i] != player {
                won_down = false;
            }
            if board[i].as_bytes()[board.len() - 1 - i] != player {
                won_up = false;
            }

            let mut won_row = true;
            let mut won_col = true;
            for j in 0..3 {
                if board[i].as_bytes()[j] != player {
                    won_row = false;
                }
                if board[j].as_bytes()[i] != player {
                    won_col = false;
                }
            }

            if won_row || won_col {
                return true;
            }
        }

        if won_up || won_down {
            return true;
        }

        false
    }

    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let mut x_count = 0;
        let mut o_count = 0;

        for row in board.iter() {
            for c in row.chars() {
                if c == 'O' {
                    o_count += 1;
                }
                if c == 'X' {
                    x_count += 1;
                }
            }
        }

        if o_count > x_count || x_count > o_count + 1 {
            return false;
        }

        let x_win = Self::did_win('X' as u8, &board);
        let o_win = Self::did_win('O' as u8, &board);

        if x_win && o_win {
            return false;
        }

        if x_win {
            return x_count == o_count + 1;
        }

        if o_win {
            return o_count == x_count;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type S = Solution;

    #[test]
    fn test_basic() {
        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["   ", "   ", "   "]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            true
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["O  ", "   ", "   "]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["O  ", "X  ", "X  "]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            true
        );
        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["OOO", "X  ", "X  "]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["XXX", "   ", "O  "]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["XXX", "   ", "OOO"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["XXX", "   ", "OOO"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["XXX", "  X", "OOO"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["XX ", "XX  ", "OOO"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );

        assert_eq!(
            S::valid_tic_tac_toe(
                vec!["XXX", "  O", " OO"]
                    .iter()
                    .map(|e| e.to_string())
                    .collect()
            ),
            false
        );
    }
}
