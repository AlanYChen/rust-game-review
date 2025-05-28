
pub mod stockfish_process;

pub mod event_parsing;

use stockfish_process::StockfishProcess;

pub fn run(start_pos: String, moves: String) -> Result<(), std::io::Error> {
    let mut stockfish = StockfishProcess::new()?;
    stockfish.setup_for_new_game(&start_pos)?;
    stockfish.print_board()?;

    

    Ok(())
}