use std::io::Error as IoError;
use std::process::Command;
use interactive_process::InteractiveProcess;

pub struct StockfishProcess {
    interactive_process: InteractiveProcess,
}

impl StockfishProcess {
    pub fn new() -> Result<StockfishProcess, IoError> {
        let mut command = get_stockfish_command()?;

        let proc = InteractiveProcess::new(&mut command, |line| {
            println!("Read line: {}", line.unwrap());
        })?;

        Ok(StockfishProcess { interactive_process: proc })
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

    pub fn print_board(&mut self) -> Result<(), IoError> {
        self.send("d")?;
        Ok(())
    }

    pub fn send(&mut self, data: &str) -> Result<(), IoError> {
        self.interactive_process.send(data)?;
        Ok(())
    }
}

/*
 * Accessory Functions
*/
fn get_stockfish_command() -> Result<Command, std::io::Error> {
    let on_windows = cfg!(target_os = "windows");
    let stockfish_path = if on_windows {
        "./stockfish-windows.exe"
    } else {
        "/var/task/stockfish-linux"
    };

    Ok(Command::new(stockfish_path))
}