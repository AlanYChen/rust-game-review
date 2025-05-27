// Copy & pasted from https://github.com/awslabs/aws-lambda-rust-runtime
#[allow(unused_variables)]

use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let func = service_fn(my_lambda_func);
    lambda_runtime::run(func).await?;
    Ok(())
}

const _API_KEY: &'static str = "nfY4b672";

use std::process::{Command, Stdio};
use std::io::{Read, Write};
use std::{thread, time::Duration};

async fn my_lambda_func(_event: LambdaEvent<Value>) -> Result<Value, Error> {
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

    let stockfish_path = if cfg!(target_os = "windows") {
        "./stockfish-windows.exe"
    } else {
        "/var/task/stockfish"
    };

    let mut stockfish;
    if cfg!(target_os = "windows") {
        stockfish = Command::new(stockfish_path)
            // .args(&["/k", stockfish_path])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn stockfish child process"); 
    } else {
        stockfish = Command::new(stockfish_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to spawn stockfish child process");
    };

    let mut stdin = stockfish.stdin.take().expect("failed to open stdin");
    let mut stdout = stockfish.stdout.take().expect("failed to open stdout");

    //////////////////
    let mut buf = Vec::new();
    let read_count = stdout.read(&mut buf).expect("couldn't read from stdout");
    let output = String::from_utf8_lossy(&buf);
    println!("output = {output}");
    println!("read_count: {read_count}");
    println!("buf: {buf:?}");
    // End of initial

    stdin.write_all("uci".as_bytes()).expect("failed to write to stdin");
    stdin.write_all("ucinewgame".as_bytes()).expect("failed to write to stdin");

    println!("write_all calls complete");
    stdin.flush().expect("error when flushing output to stdin");

    let mut buf = Vec::new();
    stdout.read(&mut buf).expect("couldn't read from stdout");
    let output = String::from_utf8_lossy(&buf);
    println!("output = {output}");

    // Input moves
    stdin.write_all("go depth 15".as_bytes()).expect("failed to write to stdin");
    stdin.flush().expect("error when flushing output to stdin");

    println!("about to sleep");
    thread::sleep(Duration::from_millis(500));

    let mut buf = Vec::new();
    stdout.read(&mut buf).expect("couldn't read from stdout");
    let output = String::from_utf8_lossy(&buf);
    println!("output = {output}");

    println!("Finished");
    Ok(json!({ "message": "success" }))


}

#[tokio::test]
async fn test_my_lambda_handler() {
    let input = serde_json::from_str("{\"body\": \"adf_s_e2e4 e7e5\"}").expect("failed to parse event");
    let context = lambda_runtime::Context::default();

    let event = lambda_runtime::LambdaEvent::new(input, context);

    my_lambda_func(event).await.expect("failed to handle event");
}
