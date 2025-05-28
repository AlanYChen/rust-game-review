
pub mod stockfish_process;
pub mod event_parsing;
pub mod engine_eval;
pub mod engine_output;

use stockfish_process::StockfishProcess;

pub fn run(start_pos: String, moves: String) -> Result<(), std::io::Error> {
    let mut stockfish = StockfishProcess::new()?;
    stockfish.ensure_ready()?;
    stockfish.setup_for_new_game(&start_pos)?;
    stockfish.print_board()?;

    let fen = stockfish.get_fen()?;
    println!("fen: {fen}");

    let engine_output = stockfish.go_to_depth(15)?;
    println!("engine_output: {engine_output:?}");

    Ok(())
}