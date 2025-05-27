// Copy & pasted from https://github.com/awslabs/aws-lambda-rust-runtime

use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

const API_KEY: &'static str = "nfY4b672";

use std::process::{Command, Stdio};
use std::io::Write;

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event["firstName"].as_str().unwrap_or("world");

    let a = event["firstName"].as_str();

    println!("{event:?}");
    println!("-----------------");
    println!("{event:#?}");
    println!("-----------------");

    // Actual procedure
    let args: Vec<&str> = event["body"].as_str().unwrap().split("_").collect();
    let api_key = args[0];
    let start_pos = args[1];
    let moves = args[2];

    if api_key != API_KEY {
        eprintln!("Invalid api_key: {api_key}");
    }


    // Try running stockfish
    let mut stockfish = Command::new("/var/task/stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn stockfish child process");
    let mut stdin = stockfish.stdin.take().expect("Failed to open stdin");
    
    stdin.write_all("uci".as_bytes()).expect("Failed to write to stdin");
    stdin.write_all("ucinewgame".as_bytes()).expect("Failed to write to stdin");

    let output = stockfish.wait_with_output().expect("Failed to read stdout");
    let output = String::from_utf8_lossy(&output.stdout);
    println!("output = {output}");

    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
}