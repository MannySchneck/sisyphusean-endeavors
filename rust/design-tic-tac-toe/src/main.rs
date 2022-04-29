// https://leetcode.com/problems/design-tic-tac-toe/submissions/
#[derive(Debug, PartialEq, Copy, Clone)]
enum SquareState {
    Empty,
    Player1,
    Player2,
}

#[derive(Debug)]
struct TicTacToe {
    board: Vec<SquareState>,
    stride: usize,
}

impl TicTacToe {
    fn new(n: i32) -> Self {
        let mut board = Vec::with_capacity((n * n) as usize);
        for _ in 0..n * n {
            board.push(SquareState::Empty);
        }

        TicTacToe {
            board,
            stride: n as usize,
        }
    }

    fn check_win(&self, player_token: SquareState) -> bool {
        let mut win_down_right = true;
        let mut win_up_right = true;
        for i in 0..self.stride {
            // diagonals
            if self.board[self.stride * i + i] != player_token {
                win_down_right = false;
            }
            if self.board[self.stride * i + self.stride - 1 - i] != player_token {
                win_up_right = false;
            }

            let mut win_row = true;
            let mut win_col = true;
            for j in 0..self.stride {
                if self.board[self.stride * i + j] != player_token {
                    win_row = false;
                }

                if self.board[self.stride * j + i] != player_token {
                    win_col = false;
                }
            }

            if win_row || win_col {
                return true;
            }
        }

        return win_down_right || win_up_right;
    }

    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let row = row as usize;
        let col = col as usize;

        let move_token = match player {
            1 => SquareState::Player1,
            2 => SquareState::Player2,
            _ => panic!("unknown player"),
        };

        let loc = row * self.stride + col;
        match self.board[loc] {
            SquareState::Empty => self.board[loc] = move_token,
            _ => panic!("tried to move twice!"),
        };

        if self.check_win(move_token) {
            player
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TTT = TicTacToe;

    #[test]
    fn test_basic() {
        let t = TTT::new(3);

        assert_eq!(t.board.len(), 9);
        for s in t.board {
            assert_eq!(s, SquareState::Empty);
        }
    }

    #[test]
    fn test_move() {
        let mut t = TTT::new(3);
        assert_eq!(t.make_a_move(1, 1, 1), 0);
        assert_eq!(t.make_a_move(1, 2, 2), 0);
        assert_eq!(t.board[4], SquareState::Player1);
        assert_eq!(t.board[5], SquareState::Player2);

        assert_eq!(t.make_a_move(0, 0, 1), 0);
        assert_eq!(t.make_a_move(2, 2, 1), 1);
    }
}
