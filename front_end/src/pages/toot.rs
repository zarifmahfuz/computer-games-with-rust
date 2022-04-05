use std::collections::HashMap;

#[derive(Clone)]
pub struct Game {
    pub p1: String,
    pub p2: String,
    pub winner: String,
    pub total_move: i64,
    pub max_ai_depth: u32,
    pub h_map: HashMap<usize,usize>,
    pub play_with_computer: bool,
    pub num_row:usize,
    pub num_col:usize,
}

impl Game {
    pub fn new(row_size: usize, col_size: usize, with_ai: bool, p1: String, p2: String, max_depth: u32,
    ) -> Game {
        let mut game = Game {
            p1: p1,
            p2: p2,
            winner: "".to_string(),
            total_move: 0,
            max_ai_depth: max_depth,
            h_map: HashMap::new(),
            play_with_computer: false,
            num_row: row_size,
            num_col: col_size,
        };
        for col in 0..game.num_col {
            game.h_map.insert(col,0);
        }
        game
    }
}
