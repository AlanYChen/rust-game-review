use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use serde_json::{json, Value};
use base64::prelude::*;

use rust_game_review_lambda::event_parsing::parse_event;
use rust_game_review_lambda::run;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }

    let func = service_fn(my_lambda_func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_lambda_func(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let (start_pos, moves) = parse_event(event).expect("API key should match");
    let game_review = run(start_pos, moves)?;

    let json = json!(game_review);
    Ok(json)
}

#[tokio::test]
async fn test_my_lambda_handler() {
    // let str = "{\"body\": \"nfY4b672_s_e2e4 e7e5\"}";

    let api_key = "nfY4b672";
    let starting_position = "s";
    let moves = "e2e4 e7e5";
    let body = format!("{}_{}_{}", api_key, starting_position, moves);

    let body = BASE64_STANDARD.encode(body);

    let str = String::from("{\"body\": \"") + &body + "\"}";

    let input = serde_json::from_str(&str).expect("failed to parse event");
    let context = lambda_runtime::Context::default();

    let event = lambda_runtime::LambdaEvent::new(input, context);

    my_lambda_func(event).await.expect("failed to handle event");
}