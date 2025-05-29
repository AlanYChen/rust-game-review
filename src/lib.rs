pub mod stockfish_process;
pub mod event_parsing;
pub mod engine_eval;
pub mod engine_output;
pub mod game_review;
pub mod move_annotation;
pub mod chess_analysis;

use std::io;
use game_review::{get_game_review, GameReview};

pub fn run(start_pos: String, moves: String) -> io::Result<GameReview> {
    let game_review = get_game_review(start_pos, moves)?;
    Ok(game_review)
}