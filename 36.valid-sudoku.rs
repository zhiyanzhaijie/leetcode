// @leet start
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // vec_grid[i][j] means the i grid includes number j
        let mut vec_grid: [[bool; 9]; 9] = [[false; 9]; 9];
        // vec_row[i][j] means the i row includes number j
        let mut vec_row: [[bool; 9]; 9] = [[false; 9]; 9];
        // vec_row[i][j] means the i column includes number j
        let mut vec_col: [[bool; 9]; 9] = [[false; 9]; 9];

        // iterate each row(col)
        for (i, row_vec) in board.iter().enumerate() {
            for (j, ch) in row_vec.iter().enumerate() {
                let val = match ch {
                    &n if n.is_ascii_digit() => n.to_digit(10).unwrap_or(0),
                    _ => 0,
                };

                if val != 0 {
                    let idx = (val - 1) as usize;

                    let grid_idx = (i / 3) * 1 + (j / 3) * 3;

                    // check
                    if (vec_grid[grid_idx][idx] || vec_row[i][idx] || vec_col[j][idx]) {
                        return false;
                    }

                    // mark
                    vec_grid[grid_idx][idx] = true;
                    vec_row[i][idx] = true;
                    vec_col[j][idx] = true;
                }
            }
        }

        true
    }
}
// @leet end
