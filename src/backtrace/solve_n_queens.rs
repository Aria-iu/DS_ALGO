fn is_valid(row: usize, col: usize, chessboard: &mut Vec<Vec<char>>, n: usize) -> bool {
    let mut i = 0 as usize;
    while i < row {
        if chessboard[i][col] == 'Q' {
            return false;
        }
        i += 1;
    }
    let (mut i, mut j) = (row as i32 - 1, col as i32 - 1);
    while i >= 0 && j >= 0 {
        if chessboard[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }
    let (mut i, mut j) = (row as i32 - 1, col as i32 + 1);
    while i >= 0 && j < n as i32 {
        if chessboard[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }
    return true;
}

fn backtracking(
    result: &mut Vec<Vec<String>>,
    n: usize,
    row: usize,
    chessboard: &mut Vec<Vec<char>>,
) {
    if row == n {
        let mut chessboard_clone: Vec<String> = Vec::new();
        for i in chessboard {
            chessboard_clone.push(i.iter().collect::<String>());
        }
        result.push(chessboard_clone);
        return;
    }
    for col in 0..n {
        if is_valid(row, col, chessboard, n) {
            chessboard[row][col] = 'Q';
            backtracking(result, n, row + 1, chessboard);
            chessboard[row][col] = '.';
        }
    }
}

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut chessboard: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
    backtracking(&mut result, n as usize, 0, &mut chessboard);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (k, expected) = $test_case;
                    //assert_eq!(find_itinerary(k), expected);
                    println!("{:?}",solve_n_queens(k));
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (5, vec![
                vec![1, 2, 2], vec![2, 1, 2], vec![2, 2, 1]
            ]),
    }
}
