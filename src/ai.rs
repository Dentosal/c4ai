use std::*;
use c4::*;

extern crate rand;
use rand::{Rng, ThreadRng};

pub fn get_winning_move(original: &Board) -> Option<usize> {
    for column in 0..7 {
        if original.column_get_free_space(column) > 0 {
            let mut clone = *original;
            clone.column_put(column, original.turn);
            if clone.has_won(clone.turn) {
                return Some(column);
            }
        }
    }
    None
}

pub fn get_best_move(board: &Board) -> usize {
    match get_winning_move(&board) {
        Some(x) => x,
        None    => {
            monte_carlo(&board, board.turn)

            // let mut rng = rand::thread_rng();
            // let mut col: usize;
            // col = rng.gen_range(0, 7) as usize;
            // while board.column_get_free_space(col) == 0 {
            //     col = rng.gen_range(0, 7) as usize;
            // }
            // col
        },
    }
}

pub fn get_random_move(board: &Board) -> usize {
    let mut rng = rand::thread_rng();
    let mut col: usize;
    col = rng.gen_range(0, 7) as usize;
    while board.column_get_free_space(col) == 0 {
        col = rng.gen_range(0, 7) as usize;
    }
    col
}

pub fn monte_carlo(original: &Board, player_color: Player) -> usize {
    // https://en.wikipedia.org/wiki/Monte_Carlo_tree_search
    let mut scores: [Option<i32>; 7] = [None; 7];
    for column in 0..7 {
        if original.column_get_free_space(column) == 0 {
            continue;
        }
        let mut round_results = Vec::new();
        for i in 0..1000 { // play 1000 times before taking averages
            let mut clone = *original;
            clone.put(column);
            clone.next_turn();
            while !clone.is_over() {
                let rnd_move = get_random_move(&clone);
                clone.put(rnd_move);
                clone.next_turn();
            }
            round_results.push(
                if clone.is_full() { 1 }
                else if clone.has_won(player_color) { 2 }
                else { 0 }
            );
        }
        // score this move
        let mut res = [0i32; 3]; // [lost, drawn, won]
        for r in round_results {
            res[r] += 1;
        }
        let score: i32 = res[2]-res[0];
        scores[column] = Some(score);
    }
    scores.into_iter().enumerate().max_by_key((|k| (*k).1)).unwrap().0 // unwrap cannot fail, because this function is not called with empty board
}


pub fn get_move_scores(board: &Board, player_color: Player, depth: u64) -> [u8; 7] {
    // brute force
    if depth >= 6 { // six moves forward is good enough
        return [0; 7];
    }
    let mut results: [u8; 7] = [0; 7]; // 0: not possible or could not solve, 1: lose, 2: draw, 3: win
    for column in 0..7 {
        if board.column_get_free_space(column) > 0 {
            let mut clone = *board;
            clone.column_put(column, board.turn);
            results[column] =
                if clone.is_over() {
                    if clone.is_full() { 2 }
                    else if clone.has_won(player_color) { 3 }
                    else { 1 }
                }
                else {
                    clone.next_turn();
                    let res = get_move_scores(&clone, player_color, depth+1);
                    let mut max_res = 0;
                    for i in 0..7 {
                        max_res = cmp::max(max_res, res[i]);
                    }
                    max_res
                };
        }
    }
    results
}
