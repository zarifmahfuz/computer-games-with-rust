use std::collections::HashMap;

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
    predict_grid1: Vec<Vec<i32>>,
    predict_grid2: Vec<Vec<i32>>,
    count_max_three_consec: i32,
    count_min_three_consec: i32,
    ai_search_height: i32,
    against_ai: bool,
    player_1: String,
    player_2: String,
}
////////////////////////
// NOTE!!!!!!!!!!!
// THIS IS A TOOT-OTTO game 
// NOT CONNECT-4 ANYMORE
////////////////////////
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
            predict_grid1: vec![vec![0; cols as usize]; rows as usize],
            predict_grid2: vec![vec![0; cols as usize]; rows as usize],
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
            // self.predict_grid[select_row][col] = self.min;
            self.to_move = -self.to_move;
            self.moves_made += 1;
            return (select_row as i32, col as i32);
        }
        (-1, -1)
    }

    // returns (x, y) if move was successfully made, (-1, -1) if unsuccessful
    pub fn player_2_move(&mut self, col: usize) -> (i32, i32) {
        // println!("I'm here very beginning");
        if self.to_move != self.max {
            panic!("It's not MAX(player 2)'s turn to move");
        }
        if !self.check_bounds(col) {
            return (-1, -1);
        }
        let mut TO_flag = 1;
        let col_to_move: usize;
        if self.against_ai {
            let value = minimax(self, self.ai_search_height, -(i32::MAX), i32::MAX);
            if value.1 < 0 {
                panic!("Minimax returned negative column index!")
            }
            col_to_move = value.1 as usize;
            if value.2 == 1 {
                TO_flag = 1;
            }
            else {
                TO_flag = 0
            }
        } else {
            // user requested to place a chip in a column that is full!
            if self.grid[0][col] != self.empty {
                return (-1, -1);
            }
            col_to_move = col;
        }
        // println!("I'm here before actual placement");
        let select_row = self.find_row(col_to_move);
        if self.against_ai && TO_flag == 1 {
            self.grid[select_row][col_to_move] = self.max;
        }
        else if self.against_ai && TO_flag == 0 {
            self.grid[select_row][col_to_move] = self.min;
        }
        else {
            self.grid[select_row][col_to_move] = self.max;
        }
        // self.predeict_grid[select_row][col_to_move] = self.max;
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
        // let mut horiz_right_score: i32;
        // let mut vert_down_score: i32;
        // let mut diag_bottom_right_score: i32;
        // let mut diag_top_right_score: i32;

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
                // OTTO
                if right[0] == 1 && right[1] == -1 && right[2] == -1 && right[3] == 1 {
                    return self.max;
                    // TOOT
                } else if right[0] == -1 && right[1] == 1 && right[2] == 1 && right[3] == -1 {
                    return self.min;
                    // OTTO
                } else if bottom[0] == 1 && bottom[1] == -1 && bottom[2] == -1 && bottom[3] == 1 {
                    return self.max;
                    // TOOT
                } else if bottom[0] == -1 && bottom[1] == 1 && bottom[2] == 1 && bottom[3] == -1 {
                    return self.min;
                    // OTTO
                } else if bottom_right[0] == 1
                    && bottom_right[1] == -1
                    && bottom_right[2] == -1
                    && bottom_right[3] == 1
                {
                    return self.max;
                    // TOOT
                } else if bottom_right[0] == -1
                    && bottom_right[1] == 1
                    && bottom_right[2] == 1
                    && bottom_right[3] == -1
                {
                    return self.min;
                    // OTTO
                } else if top_right[0] == 1
                    && top_right[1] == -1
                    && top_right[2] == -1
                    && top_right[3] == 1
                {
                    return self.max;
                    // TOOT
                } else if top_right[0] == -1
                    && top_right[1] == 1
                    && top_right[2] == 1
                    && top_right[3] == -1
                {
                    return self.min;
                }
            }
        }
        // no winner
        return 0;
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

    // T
    // returns true if move has been made, returns false if move is invalid
    fn ai_make_move1(&mut self, col: usize) -> bool {
        if !self.check_bounds(col) {
            return false;
        }
        // if column is not completely filled
        if self.grid[0][col] == self.empty {
            // find the highest row index that is empty
            let select_row = self.find_row(col);
            
            self.grid[select_row][col] = self.max;      // place a chip in the chosen cell
            self.to_move = -self.to_move;                   // alternate players
            self.moves_made += 1;
            return true;
        }
        return false;
    }
    // O
    // returns true if move has been made, returns false if move is invalid
    fn ai_make_move2(&mut self, col: usize) -> bool {
        if !self.check_bounds(col) {
            return false;
        }
        // if column is not completely filled
        if self.grid[0][col] == self.empty {
            // find the highest row index that is empty
            let select_row = self.find_row(col);
            
            self.grid[select_row][col] = self.min;      // place a chip in the chosen cell
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
        let disp = vec!['T', '-', 'O'];     // min - empty - max
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
fn minimax(st: &mut Connect4State, height: i32, mut alpha: i32, mut beta: i32) -> (i32, i32, i32) {
    // if we can't recurse any deeper, we need to estimate this non-terminal state
    if height == 0 /* || height >= (st.size as i32 - st.moves_made) */ {
        return (st.tab_score(st.max), -1, -1);
    }
    if st.get_to_move() == st.max {
        // println!("max to move");
        let mut score = i32::MIN;
        let mut best_col_to_move = 0;
        let mut TO_flag = 1;
        let mut final_TO = 1;
        let max_value = st.max_value();
        if max_value == st.min {
            // terminal state: min wins in the current game state
            return (score, -1, -1);
        }
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            // we don't want to change the current state
            let mut clone_st = st.clone();

            let mut clone_st1 = st.clone();
            let mut clone_st2 = st.clone();
            // default T == 1
            let mut TO_flag = 1;
            let mut value1 = (0, 0, 0);
            let mut value2 = (0, 0, 0);
            if clone_st1.ai_make_move1(j) {
                value1 = minimax(&mut clone_st1, height - 1, alpha, beta);
            }
            if clone_st2.ai_make_move2(j) {
                value2 = minimax(&mut clone_st2, height - 1, alpha, beta);
            }
            if value1.0 > value2.0 {
                TO_flag = 1;
            }
            else if value1.0 < value2.0{
                TO_flag = 0;
            }

            if TO_flag == 1{
                // println!("decide use T");
                if clone_st.ai_make_move1(j) {
                    let value = minimax(&mut clone_st, height - 1, alpha, beta);
                    if value.0 > score {
                        score = value.0;
                        best_col_to_move = j as i32;
                        final_TO = 1;
                        // println!("decide use T");
                    }
                    alpha = std::cmp::max(alpha, score);
                    if alpha >= beta {
                        break;
                    }
                }
            }
            else {
                // println!("decide to use O");
                if clone_st.ai_make_move2(j) {
                    let value = minimax(&mut clone_st, height - 1, alpha, beta);
                    if value.0 > score {
                        score = value.0;
                        best_col_to_move = j as i32;
                        final_TO = 0;
                        // println!("decide use ");
                    }
                    alpha = std::cmp::max(alpha, score);
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }
        return (score, best_col_to_move, final_TO);

    } else {
        // println!("min to move");
        // MIN to move
        let mut score = i32::MAX;
        let mut best_col_to_move = 0;
        let mut TO_flag = 1;
        let mut final_TO = 1;
        let max_value = st.max_value();
        if max_value == st.max {
            // terminal state: max wins in the current game state
            return (score, -1, -1);
        }
        for j in 0..st.cols {
            // if this column is full, skip
            if st.grid[0][j] != st.empty {
                continue;
            }
            let mut clone_st = st.clone();


            let mut clone_st1 = st.clone();
            let mut clone_st2 = st.clone();
            // default T == 1
            let mut TO_flag = 1;
            let mut value1 = (0, 0, 0);
            let mut value2 = (0, 0, 0);
            if clone_st1.ai_make_move1(j) {
                value1 = minimax(&mut clone_st1, height - 1, alpha, beta);
            }
            if clone_st2.ai_make_move2(j) {
                value2 = minimax(&mut clone_st2, height - 1, alpha, beta);
            }
            if value1.0 < value2.0 {
                TO_flag = 1;
            }
            else if value1.0 > value2.0{
                TO_flag = 0;
            }
            if TO_flag == 1 {
                // println!("decide use T");
                if clone_st.ai_make_move1(j) {
                    let value = minimax(&mut clone_st, height - 1, alpha, beta);
                    if value.0 < score {
                        score = value.0;
                        best_col_to_move = j as i32;
                        final_TO = 1;
                        println!("decide use T");
                    }
                    beta = std::cmp::min(beta, score);
                    if alpha >= beta {
                        break;
                    }
                }
            }
            else {
                // println!("decide use O");
                if clone_st.ai_make_move2(j) {
                    let value = minimax(&mut clone_st, height - 1, alpha, beta);
                    if value.0 < score {
                        score = value.0;
                        best_col_to_move = j as i32;
                        final_TO = 0;
                        println!("decide use O");
                    }
                    beta = std::cmp::min(beta, score);
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }
        return (score, best_col_to_move, final_TO);
    }
}