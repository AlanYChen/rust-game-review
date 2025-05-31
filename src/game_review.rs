use crate::move_annotation::MoveAnnotation;
use crate::chess_analysis::{get_white_win_percentages, get_move_annotations, get_accuracy_scores};

use serde::{Serialize, Serializer, ser::SerializeSeq};

use stockfish::{Stockfish, EngineOutput};

pub fn get_game_review(start_pos: String, moves: String) -> std::io::Result<GameReview> {
    let stockfish_path = get_stockfish_path();
    let moves: Vec<&str> = moves.split(" ").collect();
    
    let mut stockfish = Stockfish::new(stockfish_path)?;
    stockfish.setup_for_new_game()?;

    if start_pos != "s" {
        stockfish.set_fen_position(&start_pos)?;
    }

    let num_positions_to_analyze = moves.len() + 1;
    let mut engine_outputs: Vec<EngineOutput> = Vec::with_capacity(num_positions_to_analyze);

    let engine_output = stockfish.go()?;
    engine_outputs.push(engine_output);

    for chess_move in moves {
        stockfish.play_move(chess_move)?;
        let engine_output = stockfish.go()?;
        engine_outputs.push(engine_output);
    }

    let white_win_percentages = get_white_win_percentages(&engine_outputs);
    let move_annotations = get_move_annotations(&white_win_percentages);
    let (white_accuracy_score, black_accuracy_score) = get_accuracy_scores(&white_win_percentages);

    let game_review = GameReview {
        engine_outputs,
        move_annotations,
        white_accuracy_score,
        black_accuracy_score,
    };
    Ok(game_review)
}

/*
 * Accessory Functions
*/
fn get_stockfish_path() -> &'static str {
    let on_windows = cfg!(target_os = "windows");
    if on_windows {
        "./stockfish-windows.exe"
    } else {
        "/var/task/stockfish-linux"
    }
}

#[derive(Debug)]
pub struct GameReview {
    pub engine_outputs: Vec<EngineOutput>,
    pub move_annotations: Vec<MoveAnnotation>,
    pub white_accuracy_score: u32,
    pub black_accuracy_score: u32,
}
// impl fmt::Display for GameReview {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let str = self.engine_outputs.de + " " + &self.value().to_string();
//         write!(f, "{}", str)
//     }
// }
impl Serialize for GameReview {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        // serializer.serialize_str(&self.to_string())
        let mut seq = serializer.serialize_seq(Some(4))?;

        let engine_outputs: Vec<String> = self.engine_outputs
            .iter()
            .map(|output| output.to_string())
            .collect();

        let move_annotations: Vec<String> = self.move_annotations
            .iter()
            .map(|output| output.to_string())
            .collect();

        seq.serialize_element(&engine_outputs)?;

        seq.serialize_element(&move_annotations)?;

        seq.serialize_element(&self.white_accuracy_score)?;
        seq.serialize_element(&self.black_accuracy_score)?;
        seq.end()
    }
}