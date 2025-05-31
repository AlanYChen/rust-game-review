use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error as LambdaError};
use serde_json::{json, Value};

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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::prelude::*;

    const API_KEY: &str = "nfY4b672";

    const STARTING_POSITION: &str = "s";

    // LONG_MOVES was taken from an exhibit game between master level players
    const LONG_MOVES: &str = "e2e4 c7c5 g1f3 e7e6 d2d4 c5d4 f3d4 b8c6 d4b5 d7d6 c2c4 g8f6 b1c3 a7a6 b5a3 d6d5 c4d5 e6d5 e4d5 c6b4 f1e2 f8c5 e1g1 e8g8 e2f3 c8f5 c1g5 f8e8 d1d2 b7b5 a1d1 b4d3 a3b1 h7h6 g5h4 b5b4 c3a4 c5d6 h4g3 a8c8 b2b3 g7g5 g3d6 d8d6 g2g3 f6d7 f3g2 d6f6 a2a3 a6a5 a3b4 a5b4 d2a2 f5g6 d5d6 g5g4 a2d2 g8g7 f2f3 f6d6 f3g4 d6d4 g1h1 d7f6 f1f4 f6e4 d2d3 e4f2 f4f2 g6d3 f2d2 d4e3 d2d3 c8c1 a4b2 e3f2 b1d2 c1d1 b2d1 e8e1";
    const SHORT_MOVES: &str = "e2e4 e7e5 g1f3";

    #[tokio::test]
    async fn test_short_moves() {
        let body = format!("{}_{}_{}", API_KEY, STARTING_POSITION, SHORT_MOVES);
        
        let event = make_lambda_event(&body);
        let json_result = my_lambda_func(event).await.expect("failed to handle lambda event");
        println!("json_result: {json_result}");
    }

     #[tokio::test]
    async fn test_long_moves() {
        let body = format!("{}_{}_{}", API_KEY, STARTING_POSITION, LONG_MOVES);
        
        let event  = make_lambda_event(&body);
        let json_result = my_lambda_func(event).await.expect("failed to handle lambda event");
        println!("json_result: {json_result}");
    }

    fn make_lambda_event(body: &str) -> LambdaEvent<Value> {
        let body = BASE64_STANDARD.encode(body);

        let json_str = String::from("{\"body\": \"") + &body + "\"}";

        let input = serde_json::from_str(&json_str).expect("failed to parse event");
        let context = lambda_runtime::Context::default();

        lambda_runtime::LambdaEvent::new(input, context)
    }
}