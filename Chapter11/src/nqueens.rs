pub struct ChessBoard {
    board: Vec<Vec<bool>>,
    n: usize,
}

impl ChessBoard {
    pub fn new(n: usize) -> ChessBoard {
        ChessBoard {
            n: n,
            board: vec![vec![false; n]; n],
        }
    }

    pub fn place_queens(&mut self) -> bool {
        self.place_queens_r(0)
    }

    pub fn place_queens_r(&mut self, column: usize) -> bool  {
        if column < self.n {
            for r in 0..self.n {
                if self.is_valid(r, column) {
                    self.board[r][column] = true;
                    if self.place_queens_r(column + 1) {
                        return true;
                    }

                    self.board[r][column] = false;
                }
            }
            false
        }
        else {
            true
        }
    }

    fn is_valid(&self, row: usize, col: usize) -> bool {
        for i in 0..self.n {
            if self.board[i][col] {
                return false;
            }
            if self.board[row][i] {
                return false;
            }
        }
        let mut i = 0;
        let (mut left_lower, mut left_upper, mut right_lower, mut right_upper) =
            (true, true, true, true);

        while left_lower || left_upper || right_lower || right_upper {
            if left_upper && self.board[row - i][col - i] {
                return false;
            }
            if left_lower && self.board[row + i][col - i] {
                return false;
            }
            if right_lower && self.board[row + i][col + i] {
                return false;
            }
            if right_upper && self.board[row - i][col + i] {
                return false;
            }
            i += 1;
            left_upper = row as i64 - i as i64 >= 0 && col as i64 - i as i64 >= 0;
            left_lower = row + i < self.n && col as i64 - i as i64 >= 0;

            right_lower = row + i < self.n && col + i < self.n;
            right_upper = row as i64 - i as i64 >= 0 && col + i < self.n;
        }
        true
    }

    pub fn queen_coordinates(&self) -> Vec<(usize, usize)> {
        let mut locations = vec![];
        for r in 0..self.n {
            for c in 0..self.n {
                if self.board[r][c] {
                    locations.push((r, c));
                }
            }
        }
        locations
    }

    pub fn reset(&mut self) {
        for r in 0..self.n {
            for c in 0..self.n {
                self.board[r][c] = false;
            }
        }
    }
    pub fn print_board(&self) {
        for r in self.board.iter() {
            for c in r.iter() {
                print!(" {} ", if *c { 1 } else { 0 });
            }
            println!("");
        }
        println!("--");
    }
}
