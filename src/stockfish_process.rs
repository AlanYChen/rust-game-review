use std::io::Error as IoError;
use std::process::Command;

use std::sync::mpsc::{channel, Receiver, RecvError, Sender};

use interactive_process::InteractiveProcess;

use crate::engine_eval::{EngineEval, EvalType};
use crate::engine_output::EngineOutput;

pub struct StockfishProcess {
    interactive_process: InteractiveProcess,
    receiver: Receiver<String>,
}

impl StockfishProcess {
    pub fn new() -> Result<StockfishProcess, IoError> {
        let mut command = get_stockfish_command()?;

        let (tx, rx) = channel();

        let proc = InteractiveProcess::new(&mut command, move |line| {
            let line = line.unwrap();
            // println!("Read line: {}", line);
            tx.send(line);
        })?;

        rx.recv().expect("SHould be able to read from rx initially");

        Ok(StockfishProcess { 
            interactive_process: proc,
            receiver: rx,
        })
    }

    pub fn ensure_ready(&mut self) -> Result<(), IoError> {
        self.send("isready")?;
        while self.read_line() != "readyok" {}
        Ok(())
    }

    pub fn setup_for_new_game(&mut self, start_pos: &str) -> Result<(), IoError> {
        self.send("ucinewgame")?;

        if start_pos == "s" {
            self.send("position startpos")?;
        } else {
            let mut msg = String::from("position fen ");
            msg.push_str(&start_pos);
            self.send(&msg)?;
        }

        Ok(())
    }

    pub fn send(&mut self, data: &str) -> Result<(), IoError> {
        self.interactive_process.send(data)?;
        Ok(())
    }

    pub fn get_fen(&mut self) -> Result<String, IoError> {
        self.send("d")?;

        loop {
            let line = self.read_line();
            let segments: Vec<&str> = line.split(" ").collect();
            if segments[0] == "Fen:" {
                let result = segments.join(" ");

                // Keep reading lines until reached "Checkers", which is in the last line
                while !self.read_line().contains("Checkers") {}

                return Ok(result);
            }
        }
    }

    pub fn print_board(&mut self) -> Result<(), IoError> {
        self.send("d")?;

        let mut lines: Vec<String> = Vec::with_capacity(20);

        loop {
            let line = self.read_line();
            if line.is_empty() {
                continue;
            }

            let first_segment = line.split(" ").next().expect("Non-empty line should have segment");
            if first_segment == "Fen:" {
                break
            } else {
                lines.push(line);
            }
        }
        let combined = lines.join("\n");
        println!("{combined}");
        Ok(())
    }

    pub fn go_to_depth(&mut self, depth: u8) -> Result<EngineOutput, IoError> {
        let mut message = String::from("go depth ");
        message.push_str(&depth.to_string());
        self.send(&message)?;
        self.get_engine_output()
    }

    pub fn get_engine_output(&mut self) -> Result<EngineOutput, IoError> {
        let fen = self.get_fen()?;
        let color_multiplier = if fen.contains("w") {1} else {-1};
        // Stockfish shows advantage relative to current player. This function will instead
        // use positive to represent advantage white, and negative for advantage black.

        let mut previous_line: Option<String> = None;

        loop {
            let line = self.read_line();
            let mut segments = line.split(" ");
            let first_segment = segments.next().expect("should be able to get first segment");
            if first_segment != "bestmove" {
                previous_line = Some(line);
                continue;
            }
            
            let previous_line = previous_line.unwrap();
            let previous_segments: Vec<&str> = previous_line.split(" ").collect();

            let mut score_type = None;
            let mut score_value: Option<i8> = None;

            for (i, segment) in previous_segments.iter().enumerate() {
                if *segment == "score" {
                    score_type = Some(previous_segments[i + 1]);
                    score_value = Some(
                        previous_segments[i + 2].parse::<i8>().expect("should be able to parse score_value from stockfish info line in output")
                            * color_multiplier);
                    break;
                }
            }

            let score_type = EvalType::from_str(score_type.unwrap());
            let eval = EngineEval::new(score_type, score_value.unwrap());

            let best_move = segments.next().expect("should be able to get second segment")
                .to_owned();

            let output = EngineOutput::new(eval, best_move);
            return Ok(output);
        }
    }

    /* Private Methods */
    fn read_line(&mut self) -> String {
        self.receiver.recv().expect("Should be able to read from receiver")
    }
}

/*
 * Accessory Functions
*/
fn get_stockfish_command() -> Result<Command, IoError> {
    let on_windows = cfg!(target_os = "windows");
    let stockfish_path = if on_windows {
        "./stockfish-windows.exe"
    } else {
        "/var/task/stockfish-linux"
    };

    Ok(Command::new(stockfish_path))
}