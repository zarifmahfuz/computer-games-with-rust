mod connect4;
mod toot;

use crate::connect4::Connect4State;
use crate::toot::TootAndOttoState;

use std::io;
use std::io::Write;

fn run_connect4() {
    let rows = 6;
    let cols = 7;
    let max_search_depth = 5;
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

        print!("Computer moves: ");
        let ret = game.player_2_move(0);
        println!("in column {}", ret.1);
        game.print_state();

        winner = game.check_winner();
    }
    if winner == 1 {
        println!("Computer Wins!");
    } else if winner == -1 {
        println!("You win!");
    } else if winner == 2 {
        println!("Draw!");
    }
}

fn run_toot_and_otto() {
    let rows = 6;
    let cols = 7;
    let max_search_depth = 4;
    let mut game = TootAndOttoState::new(rows, cols, max_search_depth, true, &"Zarif".to_string(), &"Computer".to_string());
    let mut winner = game.check_winner();
    game.print_state();
    println!();

    while winner == 0 {
        println!("Enter 1 if you want to place 'T' OR -1 to place 'O'");
        io::stdout().flush().expect("flush failed!");
        let mut t_or_o = String::new();
        io::stdin()
            .read_line(&mut t_or_o)
            .expect("Failed to read line.");
        let t_or_o: i32 = t_or_o.trim().parse().unwrap();

        print!("Choose a column: ");
        io::stdout().flush().expect("flush failed!");
        let mut col = String::new();
        io::stdin()
            .read_line(&mut col)
            .expect("Failed to read line.");
        let col: usize = col.trim().parse().unwrap();

        let ret = game.player_1_move(col, t_or_o);
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

        print!("Computer places ");
        let ret = game.player_2_move(0, 1);
        println!("{} in column {}", ret.2, ret.1);
        game.print_state();

        winner = game.check_winner();
    }
    if winner == 1 {
        println!("Computer Wins!");
    } else if winner == -1 {
        println!("You win!");
    } else if winner == 2 {
        println!("Draw!");
    }
}

fn main() {
    run_toot_and_otto();
}