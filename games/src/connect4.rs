#[derive(Clone)]
pub struct Connect4State {
    max: i32,
    min: i32,
    empty: i32,
    rows: usize,
    cols: usize,
    size: usize,
    to_move: i32,
    moves_made: i32,
    grid: Vec<Vec<i32>>,
    count_max_three_consec: i32,
    count_min_three_consec: i32,
    ai_search_height: i32,
    against_ai: bool,
    player_1: String,
    player_2: String,
}

impl Connect4State {
    pub fn new(rows: usize, cols: usize, search_height: i32, against_ai: bool, p1: &String, p2: &String) -> Self {
        Self {
            max: 1,
            min: -1,
            empty: 0,
            rows,
            cols,
            size: rows * cols,
            to_move: -1,                // player_1 moves first
            moves_made: 0,
            grid: vec![vec![0; cols as usize]; rows as usize],
            count_max_three_consec: 0,
            count_min_three_consec: 0,
            ai_search_height: search_height,
            against_ai,
            player_1: p1.clone(),       // player_1 is MIN
            player_2: p2.clone(),       // player_2 is MAX
        }
    }

    // returns true for valid entry, returns false otherwise
    fn check_bounds(&self, j: usize) -> bool {
        j < self.cols
    }

    // returns (x, y) if move was successfully made, (-1, -1) if unsuccessful
    pub fn player_1_move(&mut self, col: usize) -> (i32, i32) {
        if self.to_move != self.min {
            panic!("It's not MIN(player 1)'s turn to move");
        }
        if !self.check_bounds(col) {
            return (-1, -1);
        }
        if self.grid[0][col as usize] == self.empty {
            let select_row = self.find_row(col as usize);
            self.grid[select_row][col] = self.min;
            self.to_move = -self.to_move;
            self.moves_made += 1;
            return (select_row as i32, col as i32);
        }
        (-1, -1)
    }

    // returns (x, y) if move was successfully made, (-1, -1) if unsuccessful
    pub fn player_2_move(&mut self, col: usize) -> (i32, i32) {
        if self.to_move != self.max {
            panic!("It's not MAX(player 2)'s turn to move");
        }
        if !self.check_bounds(col) {
            return (-1, -1);
        }
        
        if self.grid[0][col] == self.empty {
            let col_to_move: usize;
            if self.against_ai {
                let value = alpha_beta(self, self.ai_search_height, i32::MIN + 2, i32::MAX - 2);
                col_to_move = value.1 as usize;
            } else {
                col_to_move = col;
            }
            let select_row = self.find_row(col_to_move);
            self.grid[select_row][col_to_move] = self.max;
            self.moves_made += 1;
            self.to_move = -self.to_move;
            return (select_row as i32, col_to_move as i32);
        }
        (-1, -1)
    }

    // returns 1 if MAX won, -1 if MIN won, and 0 otherwise
    pub fn check_winner(&self) -> i32 {
        self.max_value()
    }

    // returns MAX if MAX won, MIN if MIN won, and 0 otherwise
    fn max_value(&self) -> i32 {
        let mut horiz_right_score: i32;
        let mut vert_down_score: i32;
        let mut diag_bottom_right_score: i32;
        let mut diag_top_right_score: i32;

        for i in 0..self.rows {
            for j in 0..self.cols {
                horiz_right_score = 0;
                vert_down_score = 0;
                diag_bottom_right_score = 0;
                diag_top_right_score = 0;

                for k in 0..4 {
                    // from (i, j) to the right
                    if j + k < self.cols {
                        horiz_right_score += self.grid[i][j+k];
                    }
                    // from (i, j) to the bottom
                    if i + k < self.rows {
                        vert_down_score += self.grid[i+k][j];
                    }
                    // from (i, j) to bottom right
                    if (i + k < self.rows) && (j + k < self.cols) {
                        diag_bottom_right_score += self.grid[i+k][j+k];
                    }
                    // from (i, j) to top right
                    if (i as i32 - k as i32 >= 0) && (j + k < self.cols) {
                        diag_top_right_score += self.grid[i-k][j+k];
                    }
                }

                if horiz_right_score == 4 {
                    return self.max;
                }
                if horiz_right_score == -4 {
                    return self.min;
                }
                if vert_down_score == 4 {
                    return self.max;
                }
                if vert_down_score == -4 {
                    return self.min;
                }
                if diag_bottom_right_score == 4 {
                    return self.max;
                }
                if diag_bottom_right_score == -4 {
                    return self.min;
                }
                if diag_top_right_score == 4 {
                    return self.max;
                }
                if diag_top_right_score == -4 {
                    return self.min;
                }
            }
        }
        // no winner
        return 0;
    }

    // calculates the number of three consecutive chips player MAX/MIN has
    fn update_three_consec(&mut self) {
        let mut horiz_right_score: i32;
        let mut vert_down_score: i32;
        let mut diag_bottom_right_score: i32;
        let mut diag_top_right_score: i32;

        for i in 0..self.rows {
            for j in 0..self.cols {
                horiz_right_score = 0;
                vert_down_score = 0;
                diag_bottom_right_score = 0;
                diag_top_right_score = 0;

                for k in 0..3 {
                    // from (i, j) to the right
                    if j + k < self.cols {
                        horiz_right_score += self.grid[i][j+k];
                    }
                    // from (i, j) to the bottom
                    if i + k < self.rows {
                        vert_down_score += self.grid[i+k][j];
                    }
                    // from (i, j) to bottom right
                    if (i + k < self.rows) && (j + k < self.cols) {
                        diag_bottom_right_score += self.grid[i+k][j+k];
                    }
                    // from (i, j) to top right
                    if (i as i32 - k as i32 >= 0) && (j + k < self.cols) {
                        diag_top_right_score += self.grid[i-k][j+k];
                    }
                }

                if horiz_right_score == 3 {
                    self.count_max_three_consec += 1;
                }
                if horiz_right_score == -3 {
                    self.count_min_three_consec += 1;
                }
                if vert_down_score == 3 {
                    self.count_max_three_consec += 1;
                }
                if vert_down_score == -3 {
                    self.count_min_three_consec += 1;
                }
                if diag_bottom_right_score == 3 {
                    self.count_max_three_consec += 1;
                }
                if diag_bottom_right_score == -3 {
                    self.count_min_three_consec += 1;
                }
                if diag_top_right_score == 3 {
                    self.count_max_three_consec += 1;
                }
                if diag_top_right_score == -3 {
                    self.count_min_three_consec += 1;
                }
            }
        }
    }

    pub fn get_to_move(&self) -> i32 {
        self.to_move
    }

    // assumes that there is at least one non-empty cell in the given column
    pub fn find_row(&self, col: usize) -> usize {
        let mut row = self.rows - 1;
        while self.grid[row][col] != self.empty {
            row -= 1;
        }
        row
    }

    // returns true if move has been made, returns false if move is invalid
    fn ai_make_move(&mut self, col: usize) -> bool {
        if !self.check_bounds(col) {
            return false;
        }
        // if column is not completely filled
        if self.grid[0][col] == self.empty {
            // find the highest row index that is empty
            let select_row = self.find_row(col);
            
            self.grid[select_row][col] = self.to_move;      // place a chip in the chosen cell
            self.to_move = -self.to_move;                   // alternate players
            self.moves_made += 1;
            return true;
        }
        return false;
    }

    pub fn print_state(&self) {
        let disp = vec!['o', '-', 'x'];     // min - empty - max
        for i in 0..self.rows {
            for j in 0..self.cols {
                let disp_idx = (self.grid[i][j] + 1) as usize;
                print!("{} ", disp[disp_idx]);
            }
            println!();
        }
    }
}

// returns (score, column to make move)
fn alpha_beta(st: &mut Connect4State, height: i32, mut alpha: i32, beta: i32) -> (i32, i32) {
    let max_value = st.max_value();
    // base case 1
    if max_value != 0 {
        // st is a terminal state!
        let score_max = 10000 * (st.rows as i32) * (st.cols as i32) / st.moves_made;
        if st.get_to_move() == st.max {
            return (score_max, -1);
        } else {
            return (-score_max, -1);
        }
    }
    // base case 2
    if height == 0 {
        // we can't search any furthe; we need to estimate who is the winner at this state;
        st.update_three_consec();
        let score_max = 10000 * (st.rows as i32) * (st.cols as i32) / st.moves_made;
        if st.count_max_three_consec > st.count_min_three_consec {
            // max is the winner
            return (score_max, -1);
        } else if st.count_max_three_consec < st.count_min_three_consec {
            // min is the winner
            return (-score_max, -1);
        } else {
            // estimate draw
            return (0, -1);
        }
    }

    let mut score = i32::MIN;
    let mut best_col_to_move = 0;

    // iterate over all the children
    for i in 0..st.cols {
        // we don't want to change the current state
        let mut clone_st = st.clone();
        if clone_st.ai_make_move(i) {
            let mut value = alpha_beta(&mut clone_st, height - 1, -beta, -alpha);
            // println!("Value before negation: {}", value.0);
            value.0 = -value.0;
            if value.0 > score {
                // better move
                score = value.0;
                best_col_to_move = i as i32;
                if score >= alpha {
                    alpha = score;
                }
                if score >= beta {
                    break;      // beta cut
                }
            }
        }
    }
    return (score, best_col_to_move);
}