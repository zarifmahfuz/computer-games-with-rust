#[derive(Clone)]
pub struct TootAndOttoState {
    max: i32,               // player 2 will always be MAX; MAX tries to spell OTTO
    min: i32,               // player 1 will always be MIN; MIN tries to spell TOOT
    empty: i32,
    t_val: i32,
    o_val: i32,
    rows: usize,
    cols: usize,
    size: usize,
    to_move: i32,
    moves_made: i32,
    grid: Vec<Vec<i32>>,
    ai_search_height: i32,
    against_ai: bool,
    player_1: String,
    player_2: String,
}

impl TootAndOttoState {
    pub fn new(rows: usize, cols: usize, search_height: i32, against_ai: bool, p1: &String, p2: &String) -> Self {
        Self {
            max: 1,
            min: -1,
            empty: 0,
            t_val: 1,
            o_val: -1,
            rows,
            cols,
            size: rows * cols,
            to_move: -1,                // player 1 or MIN moves first
            moves_made: 0,
            grid: vec![vec![0; cols as usize]; rows as usize],
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

    // t_or_o is 1 if move is T, -1 if move is O
    // returns (x, y, move_made) if move was successfully made, (-1, -1, 0) if unsuccessful
    pub fn player_1_move(&mut self, col: usize, t_or_o: i32) -> (i32, i32, i32) {
        if !(t_or_o == -1 || t_or_o == 1) {
            panic!("t_or_o must be either 1 or -1");
        }
        if self.to_move != self.min {
            panic!("It's not MIN(player 1)'s turn to move");
        }
        if !self.check_bounds(col) {
            return (-1, -1, 0);
        }
        if self.grid[0][col as usize] == self.empty {
            let select_row = self.find_row(col as usize);
            self.grid[select_row][col] = t_or_o;
            self.to_move = -self.to_move;
            self.moves_made += 1;
            return (select_row as i32, col as i32, t_or_o);
        }
        (-1, -1, 0)
    }

    // t_or_o is 1 if move is T, -1 if move is O
    // returns (x, y, move made) if move was successfully made, (-1, -1, 0) if unsuccessful
    pub fn player_2_move(&mut self, col: usize, mut t_or_o: i32) -> (i32, i32, i32) {
        if !(t_or_o == -1 || t_or_o == 1) {
            panic!("t_or_o must be either 1 or -1");
        }
        if self.to_move != self.max {
            panic!("It's not MAX(player 2)'s turn to move");
        }
        if !self.check_bounds(col) {
            return (-1, -1, 0);
        }
        
        let col_to_move: usize;
        if self.against_ai {
            let value = minimax(self, self.ai_search_height, -(i32::MAX), i32::MAX);
            if value.1 < 0 {
                panic!("Minimax returned negative column index!");
            }
            col_to_move = value.1 as usize;
            t_or_o = value.2;
        } else {
            // user requested to place a chip in a column that is full!
            if self.grid[0][col] != self.empty {
                return (-1, -1, 0);
            }
            col_to_move = col;
        }
        let select_row = self.find_row(col_to_move);
        self.grid[select_row][col_to_move] = t_or_o;
        self.moves_made += 1;
        self.to_move = -self.to_move;
        return (select_row as i32, col_to_move as i32, t_or_o);
    }

    // assumes that there is at least one non-empty cell in the given column
    pub fn find_row(&self, col: usize) -> usize {
        let mut row = self.rows - 1;
        while self.grid[row][col] != self.empty {
            row -= 1;
        }
        row
    }

    pub fn get_to_move(&self) -> i32 {
        self.to_move
    }

    // t_or_o is 1 if move is T, -1 if move is O
    // returns true if move has been made, returns false if move is invalid
    fn ai_make_move(&mut self, col: usize, t_or_o: i32) -> bool {
        if !self.check_bounds(col) {
            return false;
        }
        // if column is not completely filled
        if self.grid[0][col] == self.empty {
            // find the highest row index that is empty
            let select_row = self.find_row(col);
            self.grid[select_row][col] = t_or_o;      // place a chip in the chosen cell
            self.to_move = -self.to_move;             // alternate players
            self.moves_made += 1;
            return true;
        }
        return false;
    }

    // returns MAX if MAX won, MIN if MIN won, 0 if game is still running and 2 if game is a draw
    pub fn check_winner(&self) -> i32 {
        self.max_value()
    }

    // returns MAX if MAX won, MIN if MIN won, 0 if game is still running and 2 if game is a draw
    // TOOT: 1,-1,-1,1; OTTO: -1,1,1,-1
    // MIN plays for TOOT and MAX plays for OTTO
    fn max_value(&self) -> i32 {
        if self.moves_made as usize >= self.size {
            return 2;
        }

        let mut right = [0; 4];
        let mut bottom = [0; 4];
        let mut bottom_right = [0; 4];
        let mut top_right = [0; 4];

        for i in 0..self.rows {
            for j in 0..self.cols {
                for ele in 0..4 {
                    right[ele] = 0;
                    bottom[ele] = 0;
                    bottom_right[ele] = 0;
                    top_right[ele] = 0;
                }
                

                for k in 0..4 {
                    // from (i, j) to the right
                    if j + k < self.cols {
                        right[k] = self.grid[i][j+k];
                    }
                    // from (i, j) to the bottom
                    if i + k < self.rows {
                        bottom[k] = self.grid[i+k][j];
                    }
                    // from (i, j) to bottom right
                    if (i + k < self.rows) && (j + k < self.cols) {
                        bottom_right[k] = self.grid[i+k][j+k];
                    }
                    // from (i, j) to top right
                    if (i as i32 - k as i32 >= 0) && (j + k < self.cols) {
                        top_right[k] = self.grid[i-k][j+k];
                    }
                }
                
                if right[0] == 1 && right[1] == -1 && right[2] == -1 && right[3] == 1 {
                    return self.min;
                } else if right[0] == -1 && right[1] == 1 && right[2] == 1 && right[3] == -1 {
                    return self.max;
                } else if bottom[0] == 1 && bottom[1] == -1 && bottom[2] == -1 && bottom[3] == 1 {
                    return self.min;
                } else if bottom[0] == -1 && bottom[1] == 1 && bottom[2] == 1 && bottom[3] == -1 {
                    return self.max;
                } else if bottom_right[0] == 1
                    && bottom_right[1] == -1
                    && bottom_right[2] == -1
                    && bottom_right[3] == 1
                {
                    return self.min;
                } else if bottom_right[0] == -1
                    && bottom_right[1] == 1
                    && bottom_right[2] == 1
                    && bottom_right[3] == -1
                {
                    return self.max;
                } else if top_right[0] == 1
                    && top_right[1] == -1
                    && top_right[2] == -1
                    && top_right[3] == 1
                {
                    return self.min;
                } else if top_right[0] == -1
                    && top_right[1] == 1
                    && top_right[2] == 1
                    && top_right[3] == -1
                {
                    return self.max;
                }
            }
        }
        // no winner
        return 0;
    }

    pub fn print_state(&self) {
        let disp = vec!['O', '-', 'T'];     // min - empty - max
        for i in 0..self.rows {
            for j in 0..self.cols {
                let disp_idx = (self.grid[i][j] + 1) as usize;
                print!("{} ", disp[disp_idx]);
            }
            println!();
        }
    }

    fn tab_score(&self, player_to_check_againt: i32) -> i32 {
        let mut score: i32 = 0;
        let mut row: Vec<i32>;
        let mut col: Vec<i32>;

        /*
         * horizontal checks, we are looking for sequences of 4
         * containing any combination of MAX, MIN and EMPTY cells
         */
        for i in 0..self.rows {
            row = Vec::new();
            for j in 0..self.cols {
                row.push(self.grid[i][j]);
            }
            for k in 0..(self.cols - 3) {
                // construct chunks of 4
                let mut set: Vec<i32> = Vec::new();
                for l in 0..4 {
                    set.push(row[k + l]);
                }
                // update score
                score += self.score_set(set, player_to_check_againt);
            }
        }

        // vertical checks
        for j in 0..self.cols {
            col = Vec::new();
            for i in 0..self.rows {
                col.push(self.grid[i][j]);
            }
            for k in 0..(self.rows - 3) {
                // construct chunks of 4
                let mut set: Vec<i32> = Vec::new();
                for l in 0..4 {
                    set.push(col[k + l]);
                }
                // update score
                score += self.score_set(set, player_to_check_againt);
            }
        }

        // diagonal checks
        // main diagonals
        for i in 0..(self.rows - 3) {
            for j in 0..(self.cols - 3) {
                // construct chunks of 4
                let mut diag_set: Vec<i32> = Vec::new();
                for l in 0..4 {
                    diag_set.push(self.grid[i + l][j + l]);
                }
                // update score
                score += self.score_set(diag_set, player_to_check_againt);
            }
        }
        // secondary diagonals
        for i in 0..(self.rows - 3) {
            for j in 0..(self.cols - 3) {
                // construct chunks of 4
                let mut diag_set: Vec<i32> = Vec::new();
                for l in 0..4 {
                    diag_set.push(self.grid[i + 3 - l][j + l]);
                }
                // update score
                score += self.score_set(diag_set, player_to_check_againt);
            }
        }
        return score;
    }

    // heauristic function
    fn score_set(&self, set: Vec<i32>, player_to_check_againt: i32) -> i32 {
        let mut mult: i32 = 1;
        if player_to_check_againt == self.min {
            mult = -1;
        }
        // we know that the size of set is always 4!
        if set[0] == self.o_val && set[1] == self.t_val && set[2] == self.t_val && set[3] == self.o_val {
            // OTTO - 4 good points for MAX
            // preference to go for winning move vs. block
            return mult * 500001;
        }
        else if set[0] == self.t_val && set[1] == self.o_val && set[2] == self.o_val && set[3] == self.t_val {
            // TOOT - 4 bad points for MAX
            return mult * -500000;
        }
        else if set[0] == self.o_val && set[1] == self.t_val && set[2] == self.empty && set[3] == self.empty {
            // 2 good points and 2 empty points for MAX
            return mult * 500;
        }
        else if set[0] == self.t_val && set[1] == self.o_val && set[2] == self.empty && set[3] == self.empty {
            // 2 bad points and 2 empty points for MAX
            // preference to block
            return mult * -501;
        }
        else if set[0] == self.o_val && set[1] == self.t_val && set[2] == self.t_val && set[3] == self.empty {
            // 3 good points and 1 empty point for MAX
            return mult * 5000;
        }
        else if set[0] == self.t_val && set[1] == self.o_val && set[2] == self.o_val && set[3] == self.empty {
            // 3 bad points and 1 empty point for MAX
            // preference to block
            return mult * -5001;
        }
        0
    }
}

// returns (score, column to make move, move to make)
fn minimax(st: &mut TootAndOttoState, height: i32, mut alpha: i32, mut beta: i32) -> (i32, i32, i32) {
    // if we can't recurse any deeper, we need to estimate this non-terminal state
    if height == 0 /* || height >= (st.size as i32 - st.moves_made) */ {
        return (st.tab_score(st.max), -1, 0);
    }
    if st.get_to_move() == st.max {
        let mut score = i32::MIN;
        let mut best_col_to_move = 0;
        let mut move_to_make = st.t_val;
        let max_value = st.max_value();
        if max_value == st.min {
            // terminal state: min wins in the current game state
            return (score, -1, 0);
        }
        // trying placing T in all children of the current game state
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            // we don't want to change the current state
            let mut clone_st = st.clone();
            if clone_st.ai_make_move(j, st.t_val) {
                let value = minimax(&mut clone_st, height - 1, alpha, beta);
                if value.0 > score {
                    score = value.0;
                    best_col_to_move = j as i32;
                }
                alpha = std::cmp::max(alpha, score);
                if alpha >= beta {
                    break;
                }
            }
        }
        // trying placing O in all children of the current game state
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            // we don't want to change the current state
            let mut clone_st = st.clone();
            if clone_st.ai_make_move(j, st.o_val) {
                let value = minimax(&mut clone_st, height - 1, alpha, beta);
                if value.0 > score {
                    score = value.0;
                    best_col_to_move = j as i32;
                    move_to_make = st.o_val;
                }
                alpha = std::cmp::max(alpha, score);
                if alpha >= beta {
                    break;
                }
            }
        }
        return (score, best_col_to_move, move_to_make);
    } else {
        // MIN to move
        let mut score = i32::MAX;
        let mut best_col_to_move = 0;
        let mut move_to_make = st.t_val;
        let max_value = st.max_value();
        if max_value == st.max {
            // terminal state: max wins in the current game state
            return (score, -1, 0);
        }
        // try placing T in all children of the current game state
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            let mut clone_st = st.clone();
            if clone_st.ai_make_move(j, st.t_val) {
                let value = minimax(&mut clone_st, height - 1, alpha, beta);
                if value.0 < score {
                    score = value.0;
                    best_col_to_move = j as i32;
                }
                beta = std::cmp::min(beta, score);
                if alpha >= beta {
                    break;
                }
            }
        }
        // try placing O in all children of the current game state
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            let mut clone_st = st.clone();
            if clone_st.ai_make_move(j, st.o_val) {
                let value = minimax(&mut clone_st, height - 1, alpha, beta);
                if value.0 < score {
                    score = value.0;
                    best_col_to_move = j as i32;
                    move_to_make = st.o_val;
                }
                beta = std::cmp::min(beta, score);
                if alpha >= beta {
                    break;
                }
            }
        }
        return (score, best_col_to_move, move_to_make);
    }
}