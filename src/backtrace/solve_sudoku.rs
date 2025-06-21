fn is_valid(row: usize, col: usize, val: char, board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        if board[row][i] == val {
            return false;
        }
    }
    for j in 0..9 {
        if board[j][col] == val {
            return false;
        }
    }
    let start_row = (row / 3) * 3;
    let start_col = (col / 3) * 3;
    for i in start_row..(start_row + 3) {
        for j in start_col..(start_col + 3) {
            if board[i][j] == val {
                return false;
            }
        }
    }
    return true;
}

fn backtracking(board: &mut Vec<Vec<char>>) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] != '.' {
                continue;
            }
            for k in '1'..='9' {
                if is_valid(i, j, k, board) {
                    board[i][j] = k;
                    if backtracking(board) {
                        return true;
                    }
                    board[i][j] = '.';
                }
            }
            return false;
        }
    }
    return true;
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    backtracking(board);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        solve_sudoku(&mut board);
        println!("{:?}", board);
    }
}
