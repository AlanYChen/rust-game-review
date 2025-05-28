// Copy & pasted from https://github.com/awslabs/aws-lambda-rust-runtime

use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let func = service_fn(my_lambda_func);
    lambda_runtime::run(func).await?;
    Ok(())
}

const _API_KEY: &'static str = "nfY4b672";

use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn get_stockfish_process() -> Result<std::process::Child, std::io::Error> {
    let on_windows = cfg!(target_os = "windows");
    let stockfish_path = if on_windows {
        "./stockfish-windows.exe"
    } else {
        "/var/task/stockfish-linux"
    };

    Ok(if on_windows {
        Command::new(stockfish_path)
            // .args(&["/k", stockfish_path])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn stockfish child process")
    } else {
        Command::new(stockfish_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn stockfish child process")
    })
}

async fn my_lambda_func(_event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    // let (event, _context) = event.into_parts();
    // let body = event["body"].as_str().expect("event needs body field");
    // let args: Vec<&str> = body.split("_").collect();
    // let api_key = args[0];
    // let start_pos = args[1];
    // let moves = args[2];
    // if api_key != API_KEY { eprintln!("Invalid api_key: {api_key}"); }

    // Quick test-command (dir)
    // let dir_output = Command::new("cmd").args(&["/c", "dir"]).output().expect("failed to run first command");
    // let output = dir_output.stdout;
    // let output = String::from_utf8_lossy(&output);
    // println!("{output}");
    let mut stockfish = get_stockfish_process().expect("failed to run stockfish");

    let _stdin = stockfish.stdin.take().expect("failed to open stdin");
    let stdout = stockfish.stdout.take().expect("failed to open stdout");

    let mut buf = String::new();
    let mut reader = BufReader::new(stdout);
    reader.read_line(&mut buf).expect("error reading line from stdout");

    println!("buf: {buf}");

    // //////////////////
    // let mut buf: [u8; 50] = [0; 50];
    // let read_count = stdout.read(&mut buf).expect("couldn't read from stdout");
    // let output = String::from_utf8_lossy(&buf);
    // println!("output = {output}");
    // println!("read_count: {read_count}");
    // println!("buf: {buf:?}");
    Ok(json!({ "message": "success" }))
}

#[tokio::test]
async fn test_my_lambda_handler() {
    let input = serde_json::from_str("{\"body\": \"adf_s_e2e4 e7e5\"}").expect("failed to parse event");
    let context = lambda_runtime::Context::default();

    let event = lambda_runtime::LambdaEvent::new(input, context);

    my_lambda_func(event).await.expect("failed to handle event");
}
