mod connect4;

use crate::connect4::Connect4State;

use std::io;
use std::io::Write;

fn main() {
    let rows = 6;
    let cols = 7;
    let max_search_depth = 4;
    let mut game = Connect4State::new(rows, cols, max_search_depth, true, &"Zarif".to_string(), &"Computer".to_string());
    let mut winner = game.check_winner();
    game.print_state();
    println!();
    while winner == 0 {
        print!("Choose a column: ");
        io::stdout().flush().expect("flush failed!");

        let mut col = String::new();
        io::stdin()
            .read_line(&mut col)
            .expect("Failed to read line.");
        let col: usize = col.trim().parse().unwrap();

        let ret = game.player_1_move(col);
        if ret.0 == -1 {
            println!("You entered an invalid column");
            continue;
        }
        println!("Your move:");
        game.print_state();

        winner = game.check_winner();
        if winner != 0 {
            break;
        }

        println!("Computer moves: ");
        let ret = game.player_2_move(0);
        game.print_state();

        winner = game.check_winner();
    }
    if winner > 0 {
        println!("Computer Wins!");
    } else {
        println!("You win!");
    }
}