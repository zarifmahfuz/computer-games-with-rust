use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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
        
        let col_to_move: usize;
        if self.against_ai {
            let value = minimax(self, self.ai_search_height, -(i32::MAX), i32::MAX);
            if value.1 < 0 {
                panic!("Minimax returned negative column index!")
            }
            col_to_move = value.1 as usize;
        } else {
            // user requested to place a chip in a column that is full!
            if self.grid[0][col] != self.empty {
                return (-1, -1);
            }
            col_to_move = col;
        }
        let select_row = self.find_row(col_to_move);
        self.grid[select_row][col_to_move] = self.max;
        self.moves_made += 1;
        self.to_move = -self.to_move;
        return (select_row as i32, col_to_move as i32);
    }

    // returns MAX if MAX won, MIN if MIN won, 0 if game is still running and 2 if game is a draw
    pub fn check_winner(&self) -> i32 {
        self.max_value()
    }

    // returns MAX if MAX won, MIN if MIN won, 0 if game is still running and 2 if game is a draw
    fn max_value(&self) -> i32 {
        if self.moves_made as usize >= self.size {
            return 2;
        }
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

    // dead code
    // calculates the number of open three in rows that MAX and MIN has
    fn _update_three_consec(&mut self) {
        let mut case_scores: HashMap<String, i32>;

        for i in 0..self.rows {
            for j in 0..self.cols {
                // only evaluate empty cells
                if self.grid[i][j] != self.empty {
                    continue;
                }
                let cases: Vec<String> = vec!["case_1".to_string(), "case_2".to_string(), "case_3".to_string(), 
                    "case_4".to_string(), "case_5".to_string(), "case_6".to_string(), "case_7".to_string(), 
                    "case_8".to_string(), "case_9".to_string(), "case_10".to_string(), "case_11".to_string(), 
                    "case_12".to_string()];
                let init_scores = vec![0; 12];
                case_scores = cases.into_iter().zip(init_scores.into_iter()).collect();

                // in the middle along the main diagonal
                if (i as i32 - 1 >= 0) && (j as i32 - 1 >= 0) && (i + 1 < self.rows) && (j + 1 < self.cols) {
                    let case_1 = "case_1".to_string();
                    let entry = case_scores.entry(case_1).or_insert(0);
                    *entry = self.grid[i-1][j-1] + self.grid[i+1][j+1];
                }

                // in the middle along the secondary diagonal
                if (i + 1 < self.rows) && (j as i32 - 1 >= 0) && (i as i32 - 1 >= 0) && (j + 1 < self.cols) {
                    let case_2 = "case_2".to_string();
                    let entry = case_scores.entry(case_2).or_insert(0);
                    *entry = self.grid[i+1][j-1] + self.grid[i-1][j+1];
                }

                // bottom-right
                if (i + 2 < self.rows) && (j + 2 < self.cols) {
                    let case_3 = "case_3".to_string();
                    let entry = case_scores.entry(case_3).or_insert(0);
                    *entry = self.grid[i+1][j+1] + self.grid[i+2][j+2];
                }

                // bottom left
                if (i + 2 < self.rows) && (j as i32 - 2 >= 0) {
                    let case_4 = "case_4".to_string();
                    let entry = case_scores.entry(case_4).or_insert(0);
                    *entry = self.grid[i+1][j-1] + self.grid[i+2][j-2];
                }

                // top-left
                if (i as i32 - 2 >= 0) && (j as i32 - 2 >= 0) {
                    let case_5 = "case_5".to_string();
                    let entry = case_scores.entry(case_5).or_insert(0);
                    *entry = self.grid[i-1][j-1] + self.grid[i-2][j-2];
                }

                // top-right
                if (i as i32 - 2 >= 0) && (j + 2 < self.cols) {
                    let case_6 = "case_6".to_string();
                    let entry = case_scores.entry(case_6).or_insert(0);
                    *entry = self.grid[i-1][j+1] + self.grid[i-2][j+2];
                }

                // in the middle, horizontally
                if (j as i32 - 1 >= 0) && (j + 1 < self.cols) {
                    let case_7 = "case_7".to_string();
                    let entry = case_scores.entry(case_7).or_insert(0);
                    *entry = self.grid[i][j-1] + self.grid[i][j+1];
                }

                // in the middle, vertically
                if (i as i32 - 1 >= 0) && (i + 1 < self.rows) {
                    let case_8 = "case_8".to_string();
                    let entry = case_scores.entry(case_8).or_insert(0);
                    *entry = self.grid[i-1][j] + self.grid[i+1][j];
                }

                // to the right
                if j + 2 < self.cols {
                    let case_9 = "case_9".to_string();
                    let entry = case_scores.entry(case_9).or_insert(0);
                    *entry = self.grid[i][j+1] + self.grid[i][j+2];
                }

                // to the left
                if j as i32 - 2 >= 0 {
                    let case_10 = "case_10".to_string();
                    let entry = case_scores.entry(case_10).or_insert(0);
                    *entry = self.grid[i][j-1] + self.grid[i][j-2];
                }

                // to the top
                if i as i32 - 2 >= 0 {
                    let case_11 = "case_11".to_string();
                    let entry = case_scores.entry(case_11).or_insert(0);
                    *entry = self.grid[i-1][j] + self.grid[i-2][j];
                }

                // to the bottom
                if i + 2 < self.rows {
                    let case_12 = "case_12".to_string();
                    let entry = case_scores.entry(case_12).or_insert(0);
                    *entry = self.grid[i+1][j] + self.grid[i+2][j];
                }

                let max_count = case_scores.iter().filter(|&(_, v)| *v == 2).count();
                if max_count > 0 {
                    self.count_max_three_consec += 1;
                }
                let min_count = case_scores.iter().filter(|&(_, v)| *v == -2).count();
                if min_count > 0 {
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

    fn score_set(&self, set: Vec<i32>, player_to_check_againt: i32) -> i32 {
        let mut good = 0;
        let mut bad = 0;
        let mut empty = 0;
        for val in set {
            if val == player_to_check_againt {
                good += 1;
            }
            if val == self.max || val == self.min {
                bad += 1;
            }
            if val == self.empty {
                empty += 1;
            }
        }
        // bad was calculated as (bad + good), so remove good
        bad -= good;
        return self.heauristic(good, bad, empty);
    }

    fn heauristic(&self, good_points: i32, bad_points: i32, empty_points: i32) -> i32 {
        if good_points == 4 {
            // preference to go for winning move vs. block
            return 500001;
        } else if good_points == 3 && empty_points == 1 {
            return 5000;
        } else if good_points == 2 && empty_points == 2 {
            return 500;
        } else if bad_points == 2 && empty_points == 2 {
            // preference to block
            return -501;
        } else if bad_points == 3 && empty_points == 1 {
            // preference to block
            return -5001;
        } else if bad_points == 4 {
            return -500000;
        }
        0
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
fn minimax(st: &mut Connect4State, height: i32, mut alpha: i32, mut beta: i32) -> (i32, i32) {
    // if we can't recurse any deeper, we need to estimate this non-terminal state
    if height == 0 /* || height >= (st.size as i32 - st.moves_made) */ {
        return (st.tab_score(st.max), -1);
    }
    if st.get_to_move() == st.max {
        let mut score = i32::MIN;
        let mut best_col_to_move = 0;
        let max_value = st.max_value();
        if max_value == st.min {
            // terminal state: min wins in the current game state
            return (score / st.moves_made, -1);
        }
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            // we don't want to change the current state
            let mut clone_st = st.clone();
            if clone_st.ai_make_move(j) {
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
        return (score, best_col_to_move);
    } else {
        // MIN to move
        let mut score = i32::MAX;
        let mut best_col_to_move = 0;
        let max_value = st.max_value();
        if max_value == st.max {
            // terminal state: max wins in the current game state
            return (score / st.moves_made, -1);
        }
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            let mut clone_st = st.clone();
            if clone_st.ai_make_move(j) {
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
        return (score, best_col_to_move);
    }
}

/* dead code -
// returns (score, column to make move)
fn alpha_beta(st: &mut Connect4State, height: i32, mut alpha: i32, beta: i32) -> (i32, i32) {
    let max_value = st.max_value();
    // base case 1
    if max_value != 0 {
        // st is a terminal state!
        // let score_max = (st.size as i32) *  1000 * (st.rows as i32) * (st.cols as i32) / st.moves_made;
        // let score_max = i32::MAX - 3;

        // payoffs are in the view of MAX
        if st.get_to_move() == st.max {
            return (i32::MAX - 1, -1);
        } else {
            return (-(i32::MAX - 1), -1);
        }
    }
    // base case 2
    if height == 0 || height >= (st.size as i32 - st.moves_made) {
        // we can't search any further; we need to estimate who is the winner at this state;
        // st.update_three_consec();
        // let score_max = 10000 * (st.rows as i32) * (st.cols as i32) / st.moves_made;
        // println!("DEBUG: number of open three in a row for MAX = {}", st.count_max_three_consec);
        // println!("DEBUG: number of open three in a row for MIN = {}", st.count_min_three_consec);
        /*
        if st.count_max_three_consec > st.count_min_three_consec {
            // max is the winner
            let score_max = st.count_max_three_consec * 1000 * (st.rows as i32) * (st.cols as i32) / st.moves_made;
            // payoffs are in the view of MAX
            if st.get_to_move() == st.max {
                return (score_max, -1);
            } else {
                return (-score_max, -1);
            }
        } else if st.count_max_three_consec < st.count_min_three_consec {
            // min is the winner
            let score_max = st.count_min_three_consec * 1000 * (st.rows as i32) * (st.cols as i32) / st.moves_made;
            // payoffs are in the view of MAX
            if st.get_to_move() == st.max {
                return (score_max, -1);
            } else {
                return (-score_max, -1);
            }
        } else {
            // estimate draw
            return (0, -1);
        }
        */
        let score_max = st.tab_score(st.max);
        if st.get_to_move() == st.max {
            return (score_max, -1);
        } else {
            return (-score_max, -1);
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
            if height == st.ai_search_height {
                println!("DEBUG: i={}, value={}, score={}", i, value.0, score);
            }
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
*/