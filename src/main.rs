extern crate rand;

mod c4;
mod ai;

use std::io::{self, Write};
use rand::Rng;
use c4::*;

fn read_int(limit_low: u8, limit_high: u8) -> u8 {
    // read integer from user
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout.");
        let mut mode_str = String::new();
        io::stdin().read_line(&mut mode_str).expect("Failed to read line.");

        match mode_str.trim().parse::<u8>().ok() {
            Some(x) => {
                if x < limit_low || x > limit_high {
                    continue;
                }
                return x;
            },
            None => {},
        }
    }
}

fn main() {
    // init
    let mut rng = rand::thread_rng();

    // print message
    println!("");
    println!("  ____   _  _   ");
    println!(" / ___| | || |  ");
    println!("| |     | || |_ ");
    println!("| |___  |__   _|");
    println!(" \\____|    |_|  ");
    println!("");
    println!("Select mode:");
    println!("1: Player vs Player");
    println!("2: Player vs Computer");

    // read mode from user
    let mode = read_int(1, 2);


    println!("");
    println!("Selected: {}.", match mode {
        1 => "PvP",
        2 => "PvC",
        _ => panic!("Invalid mode")
    });
    println!("");

    // start game
    let mut board = create_empty_board();

    // preinit board with random buttons
    // for i in 0..20 {
    //     let mut rng = rand::thread_rng();
    //     let mut col: usize;
    //     col = rng.gen_range(0, 7) as usize;
    //     while board.column_get_free_space(col) == 0 {
    //         col = rng.gen_range(0, 7) as usize;
    //     }
    //     board.put(col);
    //     board.next_turn();
    // }
    //


    let computer_color: Player = match rng.gen::<bool>() {
        true    => Player::White,
        false   => Player::Black,
    };

    if mode == 2 {
        println!("You will play as {:?}.", computer_color.other());
    }

    loop {
        // print board and turn
        println!("");
        println!("{}", board);
        // read column to put disk to
        let column = if mode == 1 || (mode == 2 && board.turn != computer_color) { // human
            println!("{:?} to play:", board.turn);
            let mut col: usize;
            loop {
                col = (read_int(1, 7)-1) as usize;
                if board.column_get_free_space(col) != 0 {
                    break;
                }
                println!("Cannot put disk to full column.");
            }
            col
        }
        else { // ai
            println!("Computer thinks...");
            ai::get_best_move(&board)
        };

        // process
        board.put(column);

        // test end condition
        if board.is_over() {
            println!("");
            println!("{}", board);
            if board.is_full() {
                println!("Draw!");
            }
            else {
                if mode == 1 {
                    println!("{:?} wins!", board.get_winner().unwrap());
                }
                else if mode == 2 {
                    println!("{:?} wins!", match board.get_winner().unwrap() == computer_color {
                        true    => "Computer",
                        false   => "Clever human",
                    });
                }
            }
            break;
        }

        // next turn
        board.next_turn();
    }
}
